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
use crate::utility;

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

const API_ID_NOT_FOUND: &str = "requested api id not found";
const USERNAME_NOT_FOUND: &str = "requested username not found";
const KEY_IMPORT_ERR: &str = "key import error";
const DECRYPT_ERR: &str = "decrypt password error";
const ENCRYPT_ERR: &str = "encrypt key error";
const PASSWORD_MISMATCH: &str = "password does not match";
const TOKEN_NOT_FOUND: &str = "requested token not found";
const CREATE_TOKEN_ERR: &str = "error create token";
const UPDATE_TOKEN_ERR: &str = "error update token";
const DELETE_TOKEN_ERR: &str = "error delete token";
const GENERATE_TOKEN_ERR: &str = "error generate token";
const TOKEN_MISSING: &str = "missing access token";
const TOKEN_MISMATCH: &str = "refresh token is not match";
const TOKEN_UNVERIFIED: &str = "token unverified";

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
                let password = utility::decrypt_message(request.password.as_bytes(), priv_key)
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
        let result = self.auth_db.read_user_by_name(&request.name).await;
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
        let result = self.auth_db.read_user_by_name(&request.name).await;
        let (refresh_token, access_tokens) = match result {
            Ok(value) => {
                // decrypt encrypted password hash and return error if password is not verified
                let priv_der = value.private_key.clone();
                let hash = value.password.clone();
                let priv_key = utility::import_private_key(&priv_der)
                    .map_err(|_| Status::internal(KEY_IMPORT_ERR))?;
                let password = utility::decrypt_message(request.password.as_bytes(), priv_key)
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
                let (access_id, refresh_id) = self.auth_db.create_access_token(value.id, expire, &remote_ip)
                    .await
                    .map_err(|_| Status::internal(CREATE_TOKEN_ERR))?;
                let tokens: Vec<AccessTokenMap> = value.roles.iter().map(|e| AccessTokenMap {
                    api_id: e.api_id,
                    access_token: utility::generate_token(access_id, &e.role, e.access_duration, &e.access_key)
                        .unwrap_or(String::new())
                })
                .filter(|e| e.access_token != String::new())
                .collect();
                if value.roles.len() != tokens.len() {
                    return Err(Status::internal(GENERATE_TOKEN_ERR));
                }
                (refresh_id, tokens)
            },
            Err(_) => return Err(Status::not_found(USERNAME_NOT_FOUND))
        };
        Ok(Response::new(UserLoginResponse { refresh_token, access_tokens }))
    }

    async fn user_refresh(&self, request: Request<UserRefreshRequest>)
        -> Result<Response<UserRefreshResponse>, Status>
    {
        let remote_ip = request.remote_addr().map(|s| match s.ip() {
                std::net::IpAddr::V4(v) => v.octets().to_vec(),
                std::net::IpAddr::V6(v) => v.octets().to_vec()
            }).unwrap_or(Vec::new());
        let request = request.into_inner();
        let (api_id, token) = request.access_token
            .map(|s| (s.api_id, s.access_token))
            .ok_or(Status::invalid_argument(TOKEN_MISSING))?;
        let result = self.auth_db.read_api(api_id).await;
        let (access_key, token_claims) = match result {
            Ok(value) => {
                // verify access token and get token claims
                let access_key = value.access_key;
                let token_claims = utility::verify_token(&token, &access_key)
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
                if value.refresh_id == request.refresh_token && ip_match {
                    let refresh_id = self.auth_db
                        .update_access_token(token_claims.jti, Some(value.expire), None).await
                        .map_err(|_| Status::internal(UPDATE_TOKEN_ERR))?;
                    let duration = (token_claims.exp - token_claims.iat) as u32;
                    let access_token = utility::generate_token(token_claims.jti, &token_claims.sub, duration, &access_key)
                        .map_err(|_| Status::internal(GENERATE_TOKEN_ERR))?;
                    (refresh_id, access_token)
                } else {
                    return Err(Status::invalid_argument(TOKEN_MISMATCH))
                }
            },
            Err(_) => return Err(Status::not_found(TOKEN_NOT_FOUND))
        };
        let access_token = Some(AccessTokenMap { api_id, access_token });
        Ok(Response::new(UserRefreshResponse { refresh_token, access_token }))
    }

    async fn user_logout(&self, request: Request<UserLogoutRequest>)
        -> Result<Response<UserLogoutResponse>, Status>
    {
        let request = request.into_inner();
        // delete token in database
        let result = self.auth_db.read_refresh_token(&request.refresh_token).await;
        match result {
            Ok(_) => {
                self.auth_db.delete_refresh_token(&request.refresh_token).await
                    .map_err(|_| Status::internal(DELETE_TOKEN_ERR))?;
            },
            Err(_) => return Err(Status::not_found(TOKEN_NOT_FOUND))
        }
        Ok(Response::new(UserLogoutResponse { }))
    }

}
