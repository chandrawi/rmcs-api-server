use tonic::{Request, Response, Status};
use uuid::Uuid;
use rmcs_auth_db::Auth;
use rmcs_auth_api::user::user_service_server::UserService;
use rmcs_auth_api::user::{
    UserSchema, UserId, UserName, RoleId, UserUpdate, UserRole,
    UserReadResponse, UserListResponse, UserCreateResponse, UserChangeResponse
};
use crate::utility::validator::{AuthValidator, ValidatorKind};
use super::{
    USER_NOT_FOUND, USER_CREATE_ERR, USER_UPDATE_ERR, USER_DELETE_ERR, ADD_ROLE_ERR, RMV_ROLE_ERR
};

pub struct UserServer {
    pub auth_db: Auth,
    pub validator_flag: bool
}

impl UserServer {
    pub fn new(auth_db: Auth) -> Self {
        UserServer {
            auth_db,
            validator_flag: false
        }
    }
}

#[tonic::async_trait]
impl UserService for UserServer {

    async fn read_user(&self, request: Request<UserId>)
        -> Result<Response<UserReadResponse>, Status>
    {
        let extension = request.extensions();
        let request = request.get_ref();
        self.validate(extension, ValidatorKind::User(Uuid::from_slice(&request.id).unwrap_or_default())).await?;
        let result = self.auth_db.read_user(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(USER_NOT_FOUND))
        };
        Ok(Response::new(UserReadResponse { result }))
    }

    async fn read_user_by_name(&self, request: Request<UserName>)
        -> Result<Response<UserReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_user_by_name(&request.name).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(USER_NOT_FOUND))
        };
        Ok(Response::new(UserReadResponse { result }))
    }

    async fn list_user_by_role(&self, request: Request<RoleId>)
        -> Result<Response<UserListResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.list_user_by_role(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(USER_NOT_FOUND))
        };
        Ok(Response::new(UserListResponse { results }))
    }

    async fn create_user(&self, request: Request<UserSchema>)
        -> Result<Response<UserCreateResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_user(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            &request.name,
            &request.email,
            &request.phone,
            &request.password
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(USER_CREATE_ERR))
        };
        Ok(Response::new(UserCreateResponse { id: id.as_bytes().to_vec() }))
    }

    async fn update_user(&self, request: Request<UserUpdate>)
        -> Result<Response<UserChangeResponse>, Status>
    {
        let extension = request.extensions();
        let request = request.get_ref();
        self.validate(extension, ValidatorKind::User(Uuid::from_slice(&request.id).unwrap_or_default())).await?;
        let result = self.auth_db.update_user(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            request.name.as_deref(),
            request.email.as_deref(),
            request.phone.as_deref(),
            request.password.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(USER_UPDATE_ERR))
        };
        Ok(Response::new(UserChangeResponse { }))
    }

    async fn delete_user(&self, request: Request<UserId>)
        -> Result<Response<UserChangeResponse>, Status>
    {
        let extension = request.extensions();
        let request = request.get_ref();
        self.validate(extension, ValidatorKind::User(Uuid::from_slice(&request.id).unwrap_or_default())).await?;
        let result = self.auth_db.delete_user(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(USER_DELETE_ERR))
        };
        Ok(Response::new(UserChangeResponse { }))
    }

    async fn add_user_role(&self, request: Request<UserRole>)
        -> Result<Response<UserChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.add_user_role(
            Uuid::from_slice(&request.user_id).unwrap_or_default(),
            Uuid::from_slice(&request.role_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_ROLE_ERR))
        };
        Ok(Response::new(UserChangeResponse { }))
    }

    async fn remove_user_role(&self, request: Request<UserRole>)
        -> Result<Response<UserChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.remove_user_role(
            Uuid::from_slice(&request.user_id).unwrap_or_default(),
            Uuid::from_slice(&request.role_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_ROLE_ERR))
        };
        Ok(Response::new(UserChangeResponse { }))
    }

}

impl AuthValidator for UserServer {

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
