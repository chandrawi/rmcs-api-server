use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::role::role_service_server::RoleService;
use rmcs_auth_api::role::{
    RoleSchema, RoleId, RoleName, ApiId, UserId, RoleUpdate, RoleAccess,
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
        let request = request.into_inner();
        let result = self.auth_db.read_role(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(RoleReadResponse { result, status }))
    }

    async fn read_role_by_name(&self, request: Request<RoleName>)
        -> Result<Response<RoleReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_role_by_name(
            request.api_id,
            &request.name
        ).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(RoleReadResponse { result, status }))
    }

    async fn list_role_by_api(&self, request: Request<ApiId>)
        -> Result<Response<RoleListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_role_by_api(request.api_id).await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(RoleListResponse { result, status }))
    }

    async fn list_role_by_user(&self, request: Request<UserId>)
        -> Result<Response<RoleListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_role_by_user(request.user_id).await;
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
        let request = request.into_inner();
        let result = self.auth_db.create_role(
            request.api_id,
            &request.name,
            request.multi,
            request.ip_lock,
            request.access_duration,
            request.refresh_duration
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
        let request = request.into_inner();
        let result = self.auth_db.update_role(
            request.id,
            request.name.as_deref(),
            request.multi,
            request.ip_lock,
            request.access_duration,
            request.refresh_duration
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
        let request = request.into_inner();
        let result = self.auth_db.delete_role(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(RoleChangeResponse { status }))
    }

    async fn add_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.add_role_access(request.id, request.procedure_id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(RoleChangeResponse { status }))
    }

    async fn remove_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.remove_role_access(request.id, request.procedure_id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(RoleChangeResponse { status }))
    }

}
