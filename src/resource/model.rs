use tonic::{Request, Response, Status};
use rmcs_resource_db::{Resource, DataIndexing, DataType, ConfigType, ConfigValue};
use rmcs_resource_api::model::model_service_server::ModelService;
use rmcs_resource_api::common::{self, ResponseStatus};
use rmcs_resource_api::model::{
    ModelSchema, ModelId, ModelName, ModelCategory, ModelNameCategory, ModelUpdate, ModelTypes,
    ConfigSchema, ConfigId, ConfigUpdate,
    ModelReadResponse, ModelListResponse, ModelCreateResponse, ModelChangeResponse,
    ConfigReadResponse, ConfigListResponse, ConfigCreateResponse, ConfigChangeResponse
};

pub struct ModelServer {
    pub resource_db: Resource
}

impl ModelServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db
        }
    }
}

#[tonic::async_trait]
impl ModelService for ModelServer {

    async fn read_model(&self, request: Request<ModelId>)
        -> Result<Response<ModelReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_model(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ModelReadResponse { result, status }))
    }

    async fn list_model_by_name(&self, request: Request<ModelName>)
        -> Result<Response<ModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_name(&request.name).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ModelListResponse { results, status }))
    }

    async fn list_model_by_category(&self, request: Request<ModelCategory>)
        -> Result<Response<ModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_category(&request.category).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ModelListResponse { results, status }))
    }

    async fn list_model_by_name_category(&self, request: Request<ModelNameCategory>)
        -> Result<Response<ModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_name_category(
            &request.name,
            &request.category
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ModelListResponse { results, status }))
    }

    async fn create_model(&self, request: Request<ModelSchema>)
        -> Result<Response<ModelCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.create_model(
            DataIndexing::from(common::DataIndexing::from_i32(request.indexing).unwrap_or_default()),
            &request.category,
            &request.name,
            Some(&request.description)
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ModelCreateResponse { id, status }))
    }

    async fn update_model(&self, request: Request<ModelUpdate>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.update_model(
            request.id,
            request.indexing.map(|s| DataIndexing::from(common::DataIndexing::from_i32(s).unwrap_or_default())),
            request.category.as_deref(),
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ModelChangeResponse { status }))
    }

    async fn delete_model(&self, request: Request<ModelId>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_model(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ModelChangeResponse { status }))
    }

    async fn add_model_type(&self, request: Request<ModelTypes>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.add_model_type(
            request.id,
            request.types.into_iter().map(|e| {
                DataType::from(common::DataType::from_i32(e).unwrap_or_default())
            }).collect::<Vec<DataType>>().as_ref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ModelChangeResponse { status }))
    }

    async fn remove_model_type(&self, request: Request<ModelId>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.remove_model_type(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ModelChangeResponse { status }))
    }

    async fn read_model_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_model_config(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ConfigReadResponse { result, status }))
    }

    async fn list_model_config(&self, request: Request<ModelId>)
        -> Result<Response<ConfigListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_config_by_model(request.id).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(ConfigListResponse { results, status }))
    }

    async fn create_model_config(&self, request: Request<ConfigSchema>)
        -> Result<Response<ConfigCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.create_model_config(
            request.model_id,
            request.index,
            &request.name,
            ConfigValue::from_bytes(
                &request.config_bytes, 
                ConfigType::from(common::ConfigType::from_i32(request.config_type).unwrap_or_default())
            ),
            &request.category
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(ConfigCreateResponse { id, status }))
    }

    async fn update_model_config(&self, request: Request<ConfigUpdate>)
        -> Result<Response<ConfigChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.update_model_config(
            request.id,
            request.name.as_deref(),
            request.config_bytes.map(|s| {
                ConfigValue::from_bytes(
                    &s,
                    ConfigType::from(common::ConfigType::from_i32(request.config_type.unwrap_or_default()).unwrap_or_default())
                )
            }),
            request.category.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ConfigChangeResponse { status }))
    }

    async fn delete_model_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_model_config(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(ConfigChangeResponse { status }))
    }

}
