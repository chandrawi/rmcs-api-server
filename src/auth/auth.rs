use tonic::{Request, Response, Status};
use chrono::{Duration, Utc};
use rmcs_auth_db::Auth;
use rmcs_auth_api::auth::auth_service_server::AuthService;
use rmcs_auth_api::auth::{
    ApiKeyRequest, ApiKeyResponse, ApiLoginRequest, ApiLoginResponse,
    UserKeyRequest, UserKeyResponse, UserLoginRequest, UserLoginResponse,
    UserRefreshRequest, UserRefreshResponse, UserLogoutRequest, UserLogoutResponse,
    ProcedureMap, AccessTokenMap
};
use crate::utility::{self, token};
use super::{
    API_ID_NOT_FOUND, USERNAME_NOT_FOUND, KEY_IMPORT_ERR, DECRYPT_ERR, ENCRYPT_ERR, PASSWORD_MISMATCH,
    TOKEN_NOT_FOUND, CREATE_TOKEN_ERR, UPDATE_TOKEN_ERR, DELETE_TOKEN_ERR,
    GENERATE_TOKEN_ERR, TOKEN_MISMATCH, TOKEN_UNVERIFIED
};

pub struct AuthServer {
    pub auth_db: Auth
}

impl AuthServer {
    pub fn new(auth_db: Auth) -> Self {
        AuthServer {
            auth_db
        }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServer {

    async fn api_login_key(&self, request: Request<ApiKeyRequest>)
        -> Result<Response<ApiKeyResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_api(request.api_id).await;
        let public_key = match result {
            Ok(value) => value.public_key,
            Err(_) => return Err(Status::not_found(API_ID_NOT_FOUND))
        };
        Ok(Response::new(ApiKeyResponse { public_key }))
    }

    async fn api_login(&self, request: Request<ApiLoginRequest>)
        -> Result<Response<ApiLoginResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_api(request.api_id).await;
        let (access_key, access_procedures) = match result {
            Ok(value) => {
                // decrypt encrypted password hash and return error if password is not verified
                let priv_der = value.private_key.clone();
                let hash = value.password.clone();
                let priv_key = utility::import_private_key(&priv_der)
                    .map_err(|_| Status::internal(KEY_IMPORT_ERR))?;
                let password = utility::decrypt_message(&request.password, priv_key)
                    .map_err(|_| Status::internal(DECRYPT_ERR))?;
                utility::verify_password(&password, &hash)
                    .map_err(|_| Status::invalid_argument(PASSWORD_MISMATCH))?;
                let pub_key = utility::import_public_key(&request.public_key)
                    .map_err(|_| Status::internal(KEY_IMPORT_ERR))?;
                let access_keys = utility::encrypt_message(&value.access_key, pub_key)
                    .map_err(|_| Status::internal(ENCRYPT_ERR))?;
                let procedures = value.procedures.into_iter()
                    .map(|e| ProcedureMap { procedure: e.name, roles: e.roles })
                    .collect();
                (access_keys, procedures)
            },
            Err(_) => return Err(Status::not_found(API_ID_NOT_FOUND))
        };
        Ok(Response::new(ApiLoginResponse { access_key, access_procedures }))
    }

    async fn user_login_key(&self, request: Request<UserKeyRequest>)
        -> Result<Response<UserKeyResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_user_by_name(&request.username).await;
        let public_key = match result {
            Ok(value) => value.public_key,
            Err(_) => return Err(Status::not_found(USERNAME_NOT_FOUND))
        };
        Ok(Response::new(UserKeyResponse { public_key }))
    }

    async fn user_login(&self, request: Request<UserLoginRequest>)
        -> Result<Response<UserLoginResponse>, Status>
    {
        let mut remote_ip = request.remote_addr().map(|s| match s.ip() {
                std::net::IpAddr::V4(v) => v.octets().to_vec(),
                std::net::IpAddr::V6(v) => v.octets().to_vec()
            }).unwrap_or(Vec::new());
        let request = request.into_inner();
        let result = self.auth_db.read_user_by_name(&request.username).await;
        let (auth_token, access_tokens) = match result {
            Ok(value) => {
                // decrypt encrypted password hash and return error if password is not verified
                let priv_der = value.private_key.clone();
                let hash = value.password.clone();
                let priv_key = utility::import_private_key(&priv_der)
                    .map_err(|_| Status::internal(KEY_IMPORT_ERR))?;
                let password = utility::decrypt_message(&request.password, priv_key)
                    .map_err(|_| Status::internal(DECRYPT_ERR))?;
                utility::verify_password(&password, &hash)
                    .map_err(|_| Status::invalid_argument(PASSWORD_MISMATCH))?;
                // delete all previous token if one of the roles marked as non multi device login
                let multi = value.roles.iter().map(|e| e.multi).filter(|&e| !e).count();
                if multi > 0 {
                    self.auth_db.delete_token_by_user(value.id).await
                    .map_err(|_| Status::internal(DELETE_TOKEN_ERR))?;
                }
                let ip_lock = value.roles.iter().map(|e| e.ip_lock).filter(|&e| e).count();
                if ip_lock == 0 {
                    remote_ip = Vec::new();
                }
                // get minimum refresh duration of roles associated with the user and calculate refresh expire
                let duration = value.roles.iter().map(|e| e.refresh_duration).min().unwrap_or_default();
                let expire = Utc::now() + Duration::seconds(duration as i64);
                let mut iter_tokens = self.auth_db
                    .create_auth_token(value.id, expire, &remote_ip, value.roles.len() as u32)
                    .await
                    .map_err(|_| Status::internal(CREATE_TOKEN_ERR))?
                    .into_iter();
                let mut auth_token = String::new();
                // generate access tokens using data from user role and generated access id
                let tokens: Vec<AccessTokenMap> = value.roles.iter().map(|e| {
                    let gen = iter_tokens.next().unwrap_or_default();
                    auth_token = gen.2;
                    AccessTokenMap {
                        api_id: e.api_id,
                        access_token: token::generate_token(gen.0, &e.role, e.access_duration, &e.access_key)
                            .unwrap_or(String::new()),
                        refresh_token: gen.1
                    }
                })
                .filter(|e| e.access_token != String::new())
                .collect();
                if value.roles.len() != tokens.len() {
                    return Err(Status::internal(GENERATE_TOKEN_ERR));
                }
                (auth_token, tokens)
            },
            Err(_) => return Err(Status::not_found(USERNAME_NOT_FOUND))
        };
        Ok(Response::new(UserLoginResponse { auth_token, access_tokens }))
    }

    async fn user_refresh(&self, request: Request<UserRefreshRequest>)
        -> Result<Response<UserRefreshResponse>, Status>
    {
        let remote_ip = request.remote_addr().map(|s| match s.ip() {
                std::net::IpAddr::V4(v) => v.octets().to_vec(),
                std::net::IpAddr::V6(v) => v.octets().to_vec()
            }).unwrap_or(Vec::new());
        let request = request.into_inner();
        let result = self.auth_db.read_api(request.api_id).await;
        let (access_key, token_claims) = match result {
            Ok(value) => {
                // verify access token and get token claims
                let access_key = value.access_key;
                let token_claims = token::decode_token(&request.access_token, &access_key, false)
                    .map_err(|_| Status::internal(TOKEN_UNVERIFIED))?;
                (access_key, token_claims)
            },
            Err(_) => return Err(Status::not_found(API_ID_NOT_FOUND))
        };
        let result = self.auth_db.read_access_token(token_claims.jti).await;
        let (refresh_token, access_token) = match result {
            Ok(value) => {
                // check if remote ip match with stored login ip
                let ip_match = if value.ip == Vec::<u8>::new() {
                    true
                } else {
                    value.ip == remote_ip
                };
                // update token in database and generate new access token if refresh token match
                if value.refresh_token == request.refresh_token && ip_match {
                    let (refresh_token, _) = self.auth_db
                        .update_access_token(token_claims.jti, Some(value.expire), None).await
                        .map_err(|_| Status::internal(UPDATE_TOKEN_ERR))?;
                    let duration = (token_claims.exp - token_claims.iat) as u32;
                    let access_token = token::generate_token(token_claims.jti, &token_claims.sub, duration, &access_key)
                        .map_err(|_| Status::internal(GENERATE_TOKEN_ERR))?;
                    (refresh_token, access_token)
                } else {
                    return Err(Status::invalid_argument(TOKEN_MISMATCH))
                }
            },
            Err(_) => return Err(Status::not_found(TOKEN_NOT_FOUND))
        };
        Ok(Response::new(UserRefreshResponse { refresh_token, access_token }))
    }

    async fn user_logout(&self, request: Request<UserLogoutRequest>)
        -> Result<Response<UserLogoutResponse>, Status>
    {
        let request = request.into_inner();
        // delete token in database
        let result = self.auth_db.list_auth_token(&request.auth_token).await;
        match result {
            Ok(value) => {
                if value.len() > 0 {
                    self.auth_db.delete_auth_token(&request.auth_token).await
                        .map_err(|_| Status::internal(DELETE_TOKEN_ERR))?;
                } else {
                    return Err(Status::not_found(TOKEN_NOT_FOUND))
                }
            },
            Err(_) => return Err(Status::not_found(TOKEN_NOT_FOUND))
        }
        Ok(Response::new(UserLogoutResponse { }))
    }

}
