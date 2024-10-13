use tonic::{Request, Response, Status};
use uuid::Uuid;
use rmcs_auth_db::{Auth, ProfileMode};
use rmcs_resource_db::{DataType, DataValue};
use rmcs_auth_api::profile::profile_service_server::ProfileService;
use rmcs_auth_api::profile::{
    RoleProfileSchema, UserProfileSchema, ProfileId, RoleId, UserId, 
    RoleProfileUpdate, UserProfileUpdate, UserProfileSwap, 
    RoleProfileReadResponse, RoleProfileListResponse, UserProfileReadResponse, UserProfileListResponse,
    ProfileCreateResponse, ProfileChangeResponse
};
use crate::utility::validator::{AuthValidator, ValidatorKind};
use super::{
    PROF_NOT_FOUND, PROF_CREATE_ERR, PROF_UPDATE_ERR, PROF_DELETE_ERR
};

pub struct ProfileServer {
    pub auth_db: Auth,
    pub validator_flag: bool
}

impl ProfileServer {
    pub fn new(auth_db: Auth) -> Self {
        ProfileServer {
            auth_db,
            validator_flag: false
        }
    }
}

#[tonic::async_trait]
impl ProfileService for ProfileServer {

    async fn read_role_profile(&self, request: Request<ProfileId>)
        -> Result<Response<RoleProfileReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_role_profile(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(PROF_NOT_FOUND))
        };
        Ok(Response::new(RoleProfileReadResponse { result }))
    }

    async fn list_role_profile(&self, request: Request<RoleId>)
        -> Result<Response<RoleProfileListResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.list_role_profile_by_role(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(PROF_NOT_FOUND))
        };
        Ok(Response::new(RoleProfileListResponse { results }))
    }

    async fn create_role_profile(&self, request: Request<RoleProfileSchema>)
        -> Result<Response<ProfileCreateResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_role_profile(
            Uuid::from_slice(&request.role_id).unwrap_or_default(),
            &request.name,
            DataType::from(request.value_type),
            ProfileMode::from(request.mode)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(PROF_CREATE_ERR))
        };
        Ok(Response::new(ProfileCreateResponse { id }))
    }

    async fn update_role_profile(&self, request: Request<RoleProfileUpdate>)
        -> Result<Response<ProfileChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.update_role_profile(
            request.id,
            request.name.as_deref(),
            request.value_type.map(|x| DataType::from(x)),
            request.mode.map(|x| ProfileMode::from(x))
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(PROF_UPDATE_ERR))
        };
        Ok(Response::new(ProfileChangeResponse { }))
    }

    async fn delete_role_profile(&self, request: Request<ProfileId>)
        -> Result<Response<ProfileChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.delete_role_profile(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(PROF_DELETE_ERR))
        };
        Ok(Response::new(ProfileChangeResponse { }))
    }

    async fn read_user_profile(&self, request: Request<ProfileId>)
        -> Result<Response<UserProfileReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_user_profile(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(PROF_NOT_FOUND))
        };
        Ok(Response::new(UserProfileReadResponse { result }))
    }

    async fn list_user_profile(&self, request: Request<UserId>)
        -> Result<Response<UserProfileListResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.list_user_profile_by_user(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(PROF_NOT_FOUND))
        };
        Ok(Response::new(UserProfileListResponse { results }))
    }

    async fn create_user_profile(&self, request: Request<UserProfileSchema>)
        -> Result<Response<ProfileCreateResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_user_profile(
            Uuid::from_slice(&request.user_id).unwrap_or_default(),
            &request.name,
            DataValue::from_bytes(
                &request.value_bytes,
                DataType::from(request.value_type)
            )
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(PROF_CREATE_ERR))
        };
        Ok(Response::new(ProfileCreateResponse { id }))
    }

    async fn update_user_profile(&self, request: Request<UserProfileUpdate>)
        -> Result<Response<ProfileChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.update_user_profile(
            request.id,
            request.name.as_deref(),
            request.value_bytes.map(|s| {
                DataValue::from_bytes(
                    &s,
                    DataType::from(request.value_type.unwrap_or_default())
                )
            })
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(PROF_UPDATE_ERR))
        };
        Ok(Response::new(ProfileChangeResponse { }))
    }

    async fn delete_user_profile(&self, request: Request<ProfileId>)
        -> Result<Response<ProfileChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.delete_user_profile(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(PROF_DELETE_ERR))
        };
        Ok(Response::new(ProfileChangeResponse { }))
    }

    async fn swap_user_profile(&self, request: Request<UserProfileSwap>)
        -> Result<Response<ProfileChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.swap_user_profile(
            Uuid::from_slice(&request.user_id).unwrap_or_default(),
            &request.name,
            request.order_1 as i16,
            request.order_2 as i16
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(PROF_DELETE_ERR))
        };
        Ok(Response::new(ProfileChangeResponse { }))
    }

}

impl AuthValidator for ProfileServer {

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
