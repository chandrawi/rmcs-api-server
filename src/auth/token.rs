use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_auth_db::Auth;
use rmcs_auth_api::token::token_service_server::TokenService;
use rmcs_auth_api::token::{
    TokenSchema, AuthToken, AccessId, UserId, AuthTokenCreate, TokenUpdate,
    TokenReadResponse, TokenListResponse, TokenCreateResponse, AuthTokenCreateResponse, 
    TokenUpdateResponse, TokenChangeResponse
};

pub struct TokenServer {
    pub auth_db: Auth
}

impl TokenServer {
    pub fn new(auth_db: Auth) -> Self {
        TokenServer {
            auth_db
        }
    }
}

const TOKEN_NOT_FOUND: &str = "requested token not found";
const TOKEN_CREATE_ERR: &str = "create token error";
const TOKEN_UPDATE_ERR: &str = "update token error";
const TOKEN_DELETE_ERR: &str = "delete token error";

#[tonic::async_trait]
impl TokenService for TokenServer {

    async fn read_access_token(&self, request: Request<AccessId>)
        -> Result<Response<TokenReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_access_token(request.access_id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(TOKEN_NOT_FOUND))
        };
        Ok(Response::new(TokenReadResponse { result }))
    }

    async fn list_auth_token(&self, request: Request<AuthToken>)
        -> Result<Response<TokenListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_auth_token(&request.auth_token).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(TOKEN_NOT_FOUND))
        };
        Ok(Response::new(TokenListResponse { results }))
    }

    async fn list_token_by_user(&self, request: Request<UserId>)
        -> Result<Response<TokenListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_token_by_user(request.user_id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(TOKEN_NOT_FOUND))
        };
        Ok(Response::new(TokenListResponse { results }))
    }

    async fn create_access_token(&self, request: Request<TokenSchema>)
        -> Result<Response<TokenCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.create_access_token(
            request.user_id,
            &request.auth_token,
            Utc.timestamp_nanos(request.expire),
            request.ip.as_slice()
        ).await;
        let (access_id, refresh_token, auth_token) = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(TOKEN_CREATE_ERR))
        };
        Ok(Response::new(TokenCreateResponse { access_id, refresh_token, auth_token }))
    }

    async fn create_auth_token(&self, request: Request<AuthTokenCreate>)
        -> Result<Response<AuthTokenCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.create_auth_token(
            request.user_id,
            Utc.timestamp_nanos(request.expire),
            request.ip.as_slice(),
            request.number
        ).await;
        let tokens = match result {
            Ok(value) => value.into_iter()
                .map(|t| TokenCreateResponse {
                    access_id: t.0,
                    refresh_token: t.1,
                    auth_token: t.2
                }).collect(),
            Err(_) => return Err(Status::internal(TOKEN_CREATE_ERR))
        };
        Ok(Response::new(AuthTokenCreateResponse { tokens }))
    }

    async fn update_access_token(&self, request: Request<TokenUpdate>)
        -> Result<Response<TokenUpdateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.update_access_token(
            request.access_id.unwrap_or_default(),
            request.expire.map(|s| Utc.timestamp_nanos(s)),
            request.ip.as_deref()
        ).await;
        let (refresh_token, auth_token) = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(TOKEN_UPDATE_ERR))
        };
        Ok(Response::new(TokenUpdateResponse { refresh_token, auth_token }))
    }

    async fn update_auth_token( &self, request: Request<TokenUpdate>)
        -> Result<Response<TokenUpdateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.update_auth_token(
            request.auth_token.unwrap_or_default().as_ref(),
            request.expire.map(|s| Utc.timestamp_nanos(s)),
            request.ip.as_deref()
        ).await;
        let (refresh_token, auth_token) = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(TOKEN_UPDATE_ERR))
        };
        Ok(Response::new(TokenUpdateResponse { refresh_token, auth_token }))
    }

    async fn delete_access_token(&self, request: Request<AccessId>)
        -> Result<Response<TokenChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_access_token(request.access_id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(TOKEN_DELETE_ERR))
        };
        Ok(Response::new(TokenChangeResponse { }))
    }

    async fn delete_auth_token(&self, request: Request<AuthToken>)
        -> Result<Response<TokenChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_auth_token(&request.auth_token).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(TOKEN_DELETE_ERR))
        };
        Ok(Response::new(TokenChangeResponse { }))
    }

    async fn delete_token_by_user(&self, request: Request<UserId>)
        -> Result<Response<TokenChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_token_by_user(request.user_id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(TOKEN_DELETE_ERR))
        };
        Ok(Response::new(TokenChangeResponse { }))
    }

}
