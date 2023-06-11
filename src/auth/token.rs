use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_auth_db::Auth;
use rmcs_auth_api::token::token_service_server::TokenService;
use rmcs_auth_api::token::{
    TokenSchema, RefreshId, AccessId, UserId, TokenUpdate,
    TokenReadResponse, TokenListResponse, TokenCreateResponse, TokenUpdateResponse, TokenChangeResponse,
    ResponseStatus
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

#[tonic::async_trait]
impl TokenService for TokenServer {

    async fn read_access_token(&self, request: Request<AccessId>)
        -> Result<Response<TokenReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_access_token(request.access_id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenReadResponse { result, status }))
    }

    async fn read_refresh_token(&self, request: Request<RefreshId>)
        -> Result<Response<TokenReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_refresh_token(&request.refresh_id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenReadResponse { result, status }))
    }

    async fn list_token_by_user(&self, request: Request<UserId>)
        -> Result<Response<TokenListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_token_by_user(request.user_id).await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenListResponse { result, status }))
    }

    async fn create_access_token(&self, request: Request<TokenSchema>)
        -> Result<Response<TokenCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.create_access_token(
            request.user_id,
            Utc.timestamp_nanos(request.expire),
            request.ip.as_slice()
        ).await;
        let ((access_id, refresh_id), status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => ((0, String::new()), ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenCreateResponse { access_id, refresh_id, status }))
    }

    async fn create_refresh_token(&self, request: Request<TokenSchema>)
        -> Result<Response<TokenCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.create_refresh_token(
            request.access_id,
            request.user_id,
            Utc.timestamp_nanos(request.expire),
            request.ip.as_slice()
        ).await;
        let ((access_id, refresh_id), status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => ((0, String::new()), ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenCreateResponse { access_id, refresh_id, status }))
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
        let (refresh_id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (String::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenUpdateResponse { refresh_id, status }))
    }

    async fn update_refresh_token( &self, request: Request<TokenUpdate>)
        -> Result<Response<TokenUpdateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.update_refresh_token(
            request.refresh_id.unwrap_or_default().as_ref(),
            request.access_id,
            request.expire.map(|s| Utc.timestamp_nanos(s)),
            request.ip.as_deref()
        ).await;
        let (refresh_id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (String::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenUpdateResponse { refresh_id, status }))
    }

    async fn delete_access_token(&self, request: Request<AccessId>)
        -> Result<Response<TokenChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_access_token(request.access_id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(TokenChangeResponse { status }))
    }

    async fn delete_refresh_token(&self, request: Request<RefreshId>)
        -> Result<Response<TokenChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_refresh_token(&request.refresh_id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(TokenChangeResponse { status }))
    }

    async fn delete_token_by_user(&self, request: Request<UserId>)
        -> Result<Response<TokenChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_token_by_user(request.user_id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(TokenChangeResponse { status }))
    }

}
