use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::api::api_service_server::ApiService;
use rmcs_auth_api::api::{
    ApiSchema, ApiId, ApiName, ApiCategory, ApiUpdate,
    ProcedureSchema, ProcedureId, ProcedureName, ProcedureUpdate,
    ApiReadResponse, ApiListResponse, ApiCreateResponse, ApiChangeResponse,
    ProcedureReadResponse, ProcedureListResponse, ProcedureCreateResponse, ProcedureChangeResponse
};
use crate::utility::validator::{AuthValidator, ValidatorKind};
use super::{
    API_NOT_FOUND, API_CREATE_ERR, API_UPDATE_ERR, API_DELETE_ERR, 
    PROC_NOT_FOUND, PROC_CREATE_ERR, PROC_UPDATE_ERR, PROC_DELETE_ERR
};

pub struct ApiServer {
    pub auth_db: Auth,
    pub validator_flag: bool
}

impl ApiServer {
    pub fn new(auth_db: Auth) -> Self {
        ApiServer {
            auth_db,
            validator_flag: false
        }
    }
}

#[tonic::async_trait]
impl ApiService for ApiServer {

    async fn read_api(&self, request: Request<ApiId>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_api(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(API_NOT_FOUND))
        };
        Ok(Response::new(ApiReadResponse { result }))
    }

    async fn read_api_by_name(&self, request: Request<ApiName>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_api_by_name(&request.name).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(API_NOT_FOUND))
        };
        Ok(Response::new(ApiReadResponse { result }))
    }

    async fn list_api_by_category(&self, request: Request<ApiCategory>)
        -> Result<Response<ApiListResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.list_api_by_category(&request.category).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(API_NOT_FOUND))
        };
        Ok(Response::new(ApiListResponse { results }))
    }

    async fn create_api(&self, request: Request<ApiSchema>)
        -> Result<Response<ApiCreateResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_api(
            &request.name, 
            &request.address,
            &request.category,
            &request.description,
            &request.password
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(API_CREATE_ERR))
        };
        Ok(Response::new(ApiCreateResponse { id }))
    }

    async fn update_api(&self, request: Request<ApiUpdate>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.update_api(
            request.id,
            request.name.as_deref(),
            request.address.as_deref(),
            request.category.as_deref(),
            request.description.as_deref(),
            request.password.as_deref(),
            if request.update_key { Some(()) } else { None }
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(API_UPDATE_ERR))
        };
        Ok(Response::new(ApiChangeResponse { }))
    }

    async fn delete_api(&self, request: Request<ApiId>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.delete_api(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(API_DELETE_ERR))
        };
        Ok(Response::new(ApiChangeResponse { }))
    }

    async fn read_procedure(&self, request: Request<ProcedureId>)
        -> Result<Response<ProcedureReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_procedure(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(PROC_NOT_FOUND))
        };
        Ok(Response::new(ProcedureReadResponse { result }))
    }

    async fn read_procedure_by_name(&self, request: Request<ProcedureName>)
        -> Result<Response<ProcedureReadResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.read_procedure_by_name(
            request.api_id,
            &request.name,
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(PROC_NOT_FOUND))
        };
        Ok(Response::new(ProcedureReadResponse { result }))
    }

    async fn list_procedure_by_api(&self, request: Request<ApiId>)
        -> Result<Response<ProcedureListResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.list_procedure_by_api(request.id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(PROC_NOT_FOUND))
        };
        Ok(Response::new(ProcedureListResponse { results }))
    }

    async fn create_procedure(&self, request: Request<ProcedureSchema>)
        -> Result<Response<ProcedureCreateResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.create_procedure(
            request.api_id,
            &request.name,
            &request.description
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(PROC_CREATE_ERR))
        };
        Ok(Response::new(ProcedureCreateResponse { id }))
    }

    async fn update_procedure(&self, request: Request<ProcedureUpdate>)
        -> Result<Response<ProcedureChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.update_procedure(
            request.id,
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(PROC_UPDATE_ERR))
        };
        Ok(Response::new(ProcedureChangeResponse { }))
    }

    async fn delete_procedure(&self, request: Request<ProcedureId>)
        -> Result<Response<ProcedureChangeResponse>, Status>
    {
        self.validate(request.extensions(), ValidatorKind::Root).await?;
        let request = request.into_inner();
        let result = self.auth_db.delete_procedure(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(PROC_DELETE_ERR))
        };
        Ok(Response::new(ProcedureChangeResponse { }))
    }

}

impl AuthValidator for ApiServer {

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
