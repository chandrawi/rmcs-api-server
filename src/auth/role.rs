use tonic::{Request, Response, Status};
use uuid::Uuid;
use rmcs_auth_db::Auth;
use rmcs_auth_api::role::role_service_server::RoleService;
use rmcs_auth_api::role::{
    RoleSchema, RoleId, RoleName, ApiId, UserId, RoleUpdate, RoleAccess,
    RoleReadResponse, RoleListResponse, RoleCreateResponse, RoleChangeResponse
};
use crate::utility::validator::{AuthValidator, ValidatorKind};
use super::{
    ROLE_NOT_FOUND, ROLE_CREATE_ERR, ROLE_UPDATE_ERR, ROLE_DELETE_ERR, ADD_ACCESS_ERR, RMV_ACCESS_ERR
};

pub struct RoleServer {
    pub auth_db: Auth,
    pub validator_flag: bool
}

impl RoleServer {
    pub fn new(auth_db: Auth) -> Self {
        RoleServer {
            auth_db,
            validator_flag: false
        }
    }
}

#[tonic::async_trait]
impl RoleService for RoleServer {

    async fn read_role(&self, request: Request<RoleId>)
        -> Result<Response<RoleReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_role(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(ROLE_NOT_FOUND))
        };
        Ok(Response::new(RoleReadResponse { result }))
    }

    async fn read_role_by_name(&self, request: Request<RoleName>)
        -> Result<Response<RoleReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_role_by_name(
            Uuid::from_slice(&request.api_id).unwrap_or_default(),
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.list_role_by_api(Uuid::from_slice(&request.api_id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(ROLE_NOT_FOUND))
        };
        Ok(Response::new(RoleListResponse { results }))
    }

    async fn list_role_by_user(&self, request: Request<UserId>)
        -> Result<Response<RoleListResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.list_role_by_user(Uuid::from_slice(&request.user_id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(ROLE_NOT_FOUND))
        };
        Ok(Response::new(RoleListResponse { results }))
    }

    async fn create_role(&self, request: Request<RoleSchema>)
        -> Result<Response<RoleCreateResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_role(
            Uuid::from_slice(&request.api_id).unwrap_or_default(),
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
        Ok(Response::new(RoleCreateResponse { id: id.as_bytes().to_vec() }))
    }

    async fn update_role(&self, request: Request<RoleUpdate>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.update_role(
            Uuid::from_slice(&request.id).unwrap_or_default(),
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
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.delete_role(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ROLE_DELETE_ERR))
        };
        Ok(Response::new(RoleChangeResponse { }))
    }

    async fn add_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.add_role_access(
            Uuid::from_slice(&request.id).unwrap_or_default(), 
            Uuid::from_slice(&request.procedure_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_ACCESS_ERR))
        };
        Ok(Response::new(RoleChangeResponse { }))
    }

    async fn remove_role_access(&self, request: Request<RoleAccess>)
        -> Result<Response<RoleChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.remove_role_access(
            Uuid::from_slice(&request.id).unwrap_or_default(), 
            Uuid::from_slice(&request.procedure_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_ACCESS_ERR))
        };
        Ok(Response::new(RoleChangeResponse { }))
    }

}

impl AuthValidator for RoleServer {

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
