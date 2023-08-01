use tonic::{Request, Response, Status};
use chrono::NaiveDateTime;
use rmcs_auth_db::Auth;
use rmcs_auth_api::token::token_service_server::TokenService;
use rmcs_auth_api::token::{
    TokenSchema, AuthToken, AccessId, UserId, AuthTokenCreate, TokenUpdate,
    TokenReadResponse, TokenListResponse, TokenCreateResponse, AuthTokenCreateResponse, 
    TokenUpdateResponse, TokenChangeResponse
};
use crate::utility::validator::{AuthValidator, ValidatorKind};
use super::{
    TOKEN_NOT_FOUND, TOKEN_CREATE_ERR, TOKEN_UPDATE_ERR, TOKEN_DELETE_ERR
};

pub struct TokenServer {
    pub auth_db: Auth,
    pub validator_flag: bool
}

impl TokenServer {
    pub fn new(auth_db: Auth) -> Self {
        TokenServer {
            auth_db,
            validator_flag: false
        }
    }
}

#[tonic::async_trait]
impl TokenService for TokenServer {

    async fn read_access_token(&self, request: Request<AccessId>)
        -> Result<Response<TokenReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_access_token(
            request.user_id,
            &request.auth_token,
            NaiveDateTime::from_timestamp_micros(request.expire).unwrap_or_default(),
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_auth_token(
            request.user_id,
            NaiveDateTime::from_timestamp_micros(request.expire).unwrap_or_default(),
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.update_access_token(
            request.access_id.unwrap_or_default(),
            request.expire.map(|s| NaiveDateTime::from_timestamp_micros(s).unwrap_or_default()),
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.update_auth_token(
            request.auth_token.unwrap_or_default().as_ref(),
            request.expire.map(|s| NaiveDateTime::from_timestamp_micros(s).unwrap_or_default()),
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.delete_token_by_user(request.user_id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(TOKEN_DELETE_ERR))
        };
        Ok(Response::new(TokenChangeResponse { }))
    }

}

impl AuthValidator for TokenServer {

    fn with_validator(mut self) -> Self {
        self.validator_flag = true;
        self
    }

    fn validator_flag(&self) -> bool {
        self.validator_flag
    }

    fn auth_db(&self) ->  &Auth {
        &self.auth_db
    }

}
