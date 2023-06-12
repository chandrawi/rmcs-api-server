use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::role::role_service_server::RoleService;
use rmcs_auth_api::role::{
    RoleSchema, RoleId, RoleName, ApiId, UserId, RoleUpdate, RoleAccess,
    RoleReadResponse, RoleListResponse, RoleCreateResponse, RoleChangeResponse
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

const ROLE_NOT_FOUND: &str = "requested role not found";
const ROLE_CREATE_ERR: &str = "create role error";
const ROLE_UPDATE_ERR: &str = "update role error";
const ROLE_DELETE_ERR: &str = "delete role error";
const ADD_ACCESS_ERR: &str = "add role access error";
const RMV_ACCESS_ERR: &str = "remove role access error";

#[tonic::async_trait]
impl RoleService for RoleServer {

    async fn read_role(&self, request: Request<RoleId>)
        -> Result<Response<RoleReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_role(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(ROLE_NOT_FOUND))
        };
        Ok(Response::new(RoleReadResponse { result }))
    }

    async fn read_role_by_name(&self, request: Request<RoleName>)
        -> Result<Response<RoleReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_role_by_name(
            request.api_id,
            &request.name
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(ROLE_NOT_FOUND))
        };
        Ok(Response::new(RoleReadResponse { result }))
    }

    async fn list_role_by_api(&self, request: Request<ApiId>)
        -> Result<Response<RoleListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_role_by_api(request.api_id).await;
        let result = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(ROLE_NOT_FOUND))
        };
        Ok(Response::new(RoleListResponse { result }))
    }

    async fn list_role_by_user(&self, request: Request<UserId>)
        -> Result<Response<RoleListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_role_by_user(request.user_id).await;
        let result = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(ROLE_NOT_FOUND))
        };
        Ok(Response::new(RoleListResponse { result }))
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
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(ROLE_CREATE_ERR))
        };
        Ok(Response::new(RoleCreateResponse { id }))
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
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ROLE_UPDATE_ERR))
        };
        Ok(Response::new(RoleChangeResponse { }))
    }

    async fn delete_role(&self, request: Request<RoleId>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_role(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ROLE_DELETE_ERR))
        };
        Ok(Response::new(RoleChangeResponse { }))
    }

    async fn add_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.add_role_access(request.id, request.procedure_id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_ACCESS_ERR))
        };
        Ok(Response::new(RoleChangeResponse { }))
    }

    async fn remove_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.remove_role_access(request.id, request.procedure_id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_ACCESS_ERR))
        };
        Ok(Response::new(RoleChangeResponse { }))
    }

}
