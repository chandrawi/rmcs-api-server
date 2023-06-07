use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::api::api_service_server::ApiService;
use rmcs_auth_api::api::{
    ApiSchema, ApiId, ApiName, ApiCategory, ApiUpdate,
    ProcedureSchema, ProcedureId, ProcedureName, ProcedureUpdate,
    ApiReadResponse, ApiListResponse, ApiCreateResponse, ApiChangeResponse,
    ProcedureReadResponse, ProcedureListResponse, ProcedureCreateResponse, ProcedureChangeResponse,
    ResponseStatus
};

pub struct ApiServer {
    pub auth_db: Auth
}

impl ApiServer {
    pub fn new(auth_db: Auth) -> Self {
        ApiServer {
            auth_db
        }
    }
}

#[tonic::async_trait]
impl ApiService for ApiServer {

    async fn read_api(&self, request: Request<ApiId>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_api(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiReadResponse { result, status }))
    }

    async fn read_api_by_name(&self, request: Request<ApiName>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_api_by_name(&request.name).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiReadResponse { result, status }))
    }

    async fn list_api_by_category(&self, request: Request<ApiCategory>)
        -> Result<Response<ApiListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_api_by_category(&request.category).await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(), 
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiListResponse { result, status }))
    }

    async fn create_api(&self, request: Request<ApiSchema>)
        -> Result<Response<ApiCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.create_api(
            &request.name, 
            &request.address,
            &request.category,
            &request.description,
            request.password.as_deref().unwrap_or_default()
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiCreateResponse { id, status }))
    }

    async fn update_api(&self, request: Request<ApiUpdate>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
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
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ApiChangeResponse { status }))
    }

    async fn delete_api(&self, request: Request<ApiId>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_api(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ApiChangeResponse { status }))
    }

    async fn read_procedure(&self, request: Request<ProcedureId>)
        -> Result<Response<ProcedureReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_procedure(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ProcedureReadResponse { result, status }))
    }

    async fn read_procedure_by_name(&self, request: Request<ProcedureName>)
        -> Result<Response<ProcedureReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_procedure_by_name(
            request.api_id,
            &request.name,
        ).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ProcedureReadResponse { result, status }))
    }

    async fn list_procedure_by_api(&self, request: Request<ApiId>)
        -> Result<Response<ProcedureListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_procedure_by_api(request.id).await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(), 
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ProcedureListResponse { result, status }))
    }

    async fn create_procedure(&self, request: Request<ProcedureSchema>)
        -> Result<Response<ProcedureCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.create_procedure(
            request.api_id,
            &request.name,
            &request.description
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ProcedureCreateResponse { id, status }))
    }

    async fn update_procedure(&self, request: Request<ProcedureUpdate>)
        -> Result<Response<ProcedureChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.update_procedure(
            request.id,
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ProcedureChangeResponse { status }))
    }

    async fn delete_procedure(&self, request: Request<ProcedureId>)
        -> Result<Response<ProcedureChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_procedure(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ProcedureChangeResponse { status }))
    }

}
