use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::role::role_service_server::RoleService;
use rmcs_auth_api::role::{
    RoleSchema, RoleId, RoleName, RoleListRequest, RoleUpdate, RoleAccess,
    RoleReadResponse, RoleListResponse, RoleCreateResponse, RoleChangeResponse,
    ResponseStatus
};

pub struct RoleServer {
    pub auth_db: Auth
}

impl RoleServer {
    pub fn new(auth_db: Auth) -> Self {
        RoleServer {
            auth_db
        }
    }
}

#[tonic::async_trait]
impl RoleService for RoleServer {

    async fn read_role(&self, request: Request<RoleId>)
        -> Result<Response<RoleReadResponse>, Status>
    {
        let role_id = request.into_inner();
        let result = self.auth_db.read_role(role_id.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(RoleReadResponse { result, status }))
    }

    async fn read_role_by_name(&self, request: Request<RoleName>)
        -> Result<Response<RoleReadResponse>, Status>
    {
        let role_name = request.into_inner();
        let result = self.auth_db.read_role_by_name(&role_name.name).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(RoleReadResponse { result, status }))
    }

    async fn list_role(&self, _request: Request<RoleListRequest>)
        -> Result<Response<RoleListResponse>, Status>
    {
        let result = self.auth_db.list_role().await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(RoleListResponse { result, status }))
    }

    async fn create_role(&self, request: Request<RoleSchema>)
        -> Result<Response<RoleCreateResponse>, Status>
    {
        let role_schema = request.into_inner();
        let result = self.auth_db.create_role(
            &role_schema.name,
            role_schema.secured,
            role_schema.multi,
            Some(role_schema.token_expire),
            Some(role_schema.token_limit)
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(RoleCreateResponse { id, status }))
    }

    async fn update_role(&self, request: Request<RoleUpdate>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let role_update = request.into_inner();
        let result = self.auth_db.update_role(
            role_update.id,
            role_update.name.as_deref(),
            role_update.secured,
            role_update.multi,
            role_update.token_expire,
            role_update.token_limit
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(RoleChangeResponse { status }))
    }

    async fn delete_role(&self, request: Request<RoleId>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let role_id = request.into_inner();
        let result = self.auth_db.delete_role(role_id.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(RoleChangeResponse { status }))
    }

    async fn add_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let access = request.into_inner();
        let result = self.auth_db.add_role_access(access.id, access.procedure_id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(RoleChangeResponse { status }))
    }

    async fn remove_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let access = request.into_inner();
        let result = self.auth_db.remove_role_access(access.id, access.procedure_id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(RoleChangeResponse { status }))
    }

}
