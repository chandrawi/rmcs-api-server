use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::token::token_service_server::TokenService;
use rmcs_auth_api::token::{
    TokenSchema, TokenId, RoleId, UserId,
    TokenReadResponse, TokenListResponse, TokenCreateResponse, TokenChangeResponse,
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

    async fn read_token(&self, request: Request<TokenId>)
        -> Result<Response<TokenReadResponse>, Status>
    {
        let token_id = request.into_inner();
        let result = self.auth_db.read_token(&token_id.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenReadResponse { result, status }))
    }

    async fn list_token_by_role(&self, request: Request<RoleId>)
        -> Result<Response<TokenListResponse>, Status>
    {
        let role_id = request.into_inner();
        let result = self.auth_db.list_token_by_role(role_id.id).await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenListResponse { result, status }))
    }

    async fn list_token_by_user(&self, request: Request<UserId>)
        -> Result<Response<TokenListResponse>, Status>
    {
        let user_id = request.into_inner();
        let result = self.auth_db.list_token_by_user(user_id.id).await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(TokenListResponse { result, status }))
    }

    async fn create_token(&self, request: Request<TokenSchema>)
        -> Result<Response<TokenCreateResponse>, Status>
    {
        let token_schema = rmcs_auth_db::TokenSchema::from(request.into_inner());
        let result = self.auth_db.create_token(
            &token_schema.id,
            token_schema.role_id,
            token_schema.user_id,
            Some(token_schema.expire),
            Some(token_schema.limit),
            Some(token_schema.ip)
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(TokenCreateResponse { status }))
    }

    async fn delete_token(&self, request: Request<TokenId>)
        -> Result<Response<TokenChangeResponse>, Status>
    {
        let token_id = request.into_inner();
        let result = self.auth_db.delete_token(&token_id.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(TokenChangeResponse { status }))
    }

}
