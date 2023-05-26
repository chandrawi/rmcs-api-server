use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::api::api_service_server::ApiService;
use rmcs_auth_api::api::{
    ApiSchema, ApiId, ApiName, ApiListRequest, ApiUpdate,
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

    async fn read_resource(&self, request: Request<ApiId>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        let api_id = request.into_inner();
        let result = self.auth_db.read_resource(api_id.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiReadResponse { result, status }))
    }

    async fn read_resource_by_name(&self, request: Request<ApiName>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        let api_name = request.into_inner();
        let result = self.auth_db.read_resource_by_name(&api_name.name).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiReadResponse { result, status }))
    }

    async fn list_resource(&self, _request: Request<ApiListRequest>)
        -> Result<Response<ApiListResponse>, Status>
    {
        let result = self.auth_db.list_resource().await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(), 
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiListResponse { result, status }))
    }

    async fn create_resource(&self, request: Request<ApiSchema>)
        -> Result<Response<ApiCreateResponse>, Status>
    {
        let api_schema = request.into_inner();
        let result = self.auth_db.create_resource(
            &api_schema.name, 
            &api_schema.address,
            Some(&api_schema.description)
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiCreateResponse { id, status }))
    }

    async fn update_resource(&self, request: Request<ApiUpdate>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
        let api_update = request.into_inner();
        let result = self.auth_db.update_resource(
            api_update.id,
            api_update.name.as_deref(),
            api_update.address.as_deref(),
            api_update.description.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ApiChangeResponse { status }))
    }

    async fn delete_resource(&self, request: Request<ApiId>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
        let api_id = request.into_inner();
        let result = self.auth_db.delete_resource(api_id.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ApiChangeResponse { status }))
    }

    async fn read_application(&self, request: Request<ApiId>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        let api_id = request.into_inner();
        let result = self.auth_db.read_application(api_id.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiReadResponse { result, status }))
    }

    async fn read_application_by_name(&self, request: Request<ApiName>)
        -> Result<Response<ApiReadResponse>, Status>
    {
        let api_name = request.into_inner();
        let result = self.auth_db.read_application_by_name(&api_name.name).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiReadResponse { result, status }))
    }

    async fn list_application(&self, _request: Request<ApiListRequest>)
        -> Result<Response<ApiListResponse>, Status>
    {
        let result = self.auth_db.list_application().await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(), 
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiListResponse { result, status }))
    }

    async fn create_application(&self, request: Request<ApiSchema>)
        -> Result<Response<ApiCreateResponse>, Status>
    {
        let api_schema = request.into_inner();
        let result = self.auth_db.create_application(
            &api_schema.name, 
            &api_schema.address,
            Some(&api_schema.description)
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ApiCreateResponse { id, status }))
    }

    async fn update_application(&self, request: Request<ApiUpdate>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
        let api_update = request.into_inner();
        let result = self.auth_db.update_application(
            api_update.id,
            api_update.name.as_deref(),
            api_update.address.as_deref(),
            api_update.description.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ApiChangeResponse { status }))
    }

    async fn delete_application(&self, request: Request<ApiId>)
        -> Result<Response<ApiChangeResponse>, Status>
    {
        let api_id = request.into_inner();
        let result = self.auth_db.delete_application(api_id.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ApiChangeResponse { status }))
    }

    async fn read_procedure(&self, request: Request<ProcedureId>)
        -> Result<Response<ProcedureReadResponse>, Status>
    {
        let proc_id = request.into_inner();
        let result = self.auth_db.read_procedure(proc_id.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ProcedureReadResponse { result, status }))
    }

    async fn read_procedure_by_name(&self, request: Request<ProcedureName>)
        -> Result<Response<ProcedureReadResponse>, Status>
    {
        let proc_name = request.into_inner();
        let result = self.auth_db.read_procedure_by_name(
            proc_name.api_id,
            &proc_name.service,
            &proc_name.procedure
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
        let api_id = request.into_inner();
        let result = self.auth_db.list_procedure_by_api(api_id.id).await;
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
        let proc_schema = request.into_inner();
        let result = self.auth_db.create_procedure(
            proc_schema.api_id,
            &proc_schema.service,
            &proc_schema.procedure,
            Some(&proc_schema.description)
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
        let proc_update = request.into_inner();
        let result = self.auth_db.update_procedure(
            proc_update.id,
            proc_update.service.as_deref(),
            proc_update.procedure.as_deref(),
            proc_update.description.as_deref()
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
        let proc_id = request.into_inner();
        let result = self.auth_db.delete_procedure(proc_id.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ProcedureChangeResponse { status }))
    }

}
