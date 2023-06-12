use tonic::{Request, Response, Status};
use rmcs_resource_db::{Resource, DataIndexing, DataType, ConfigType, ConfigValue};
use rmcs_resource_api::model::model_service_server::ModelService;
use rmcs_resource_api::common;
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

const MODEL_NOT_FOUND: &str = "requested model not found";
const MODEL_CREATE_ERR: &str = "create model error";
const MODEL_UPDATE_ERR: &str = "update model error";
const MODEL_DELETE_ERR: &str = "delete model error";
const ADD_TYPE_ERR: &str = "add model type error";
const RMV_TYPE_ERR: &str = "remove model type error";
const CFG_NOT_FOUND: &str = "requested config not found";
const CFG_CREATE_ERR: &str = "create config error";
const CFG_UPDATE_ERR: &str = "update config error";
const CFG_DELETE_ERR: &str = "delete config error";

#[tonic::async_trait]
impl ModelService for ModelServer {

    async fn read_model(&self, request: Request<ModelId>)
        -> Result<Response<ModelReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_model(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(MODEL_NOT_FOUND))
        };
        Ok(Response::new(ModelReadResponse { result }))
    }

    async fn list_model_by_name(&self, request: Request<ModelName>)
        -> Result<Response<ModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(MODEL_NOT_FOUND))
        };
        Ok(Response::new(ModelListResponse { results }))
    }

    async fn list_model_by_category(&self, request: Request<ModelCategory>)
        -> Result<Response<ModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_category(&request.category).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(MODEL_NOT_FOUND))
        };
        Ok(Response::new(ModelListResponse { results }))
    }

    async fn list_model_by_name_category(&self, request: Request<ModelNameCategory>)
        -> Result<Response<ModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_name_category(
            &request.name,
            &request.category
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(MODEL_NOT_FOUND))
        };
        Ok(Response::new(ModelListResponse { results }))
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
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(MODEL_CREATE_ERR))
        };
        Ok(Response::new(ModelCreateResponse { id }))
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
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(MODEL_UPDATE_ERR))
        };
        Ok(Response::new(ModelChangeResponse { }))
    }

    async fn delete_model(&self, request: Request<ModelId>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_model(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(MODEL_DELETE_ERR))
        };
        Ok(Response::new(ModelChangeResponse { }))
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
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_TYPE_ERR))
        };
        Ok(Response::new(ModelChangeResponse { }))
    }

    async fn remove_model_type(&self, request: Request<ModelId>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.remove_model_type(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_TYPE_ERR))
        };
        Ok(Response::new(ModelChangeResponse { }))
    }

    async fn read_model_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_model_config(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(CFG_NOT_FOUND))
        };
        Ok(Response::new(ConfigReadResponse { result }))
    }

    async fn list_model_config(&self, request: Request<ModelId>)
        -> Result<Response<ConfigListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_model_config_by_model(request.id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(CFG_NOT_FOUND))
        };
        Ok(Response::new(ConfigListResponse { results }))
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
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(CFG_CREATE_ERR))
        };
        Ok(Response::new(ConfigCreateResponse { id }))
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
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(CFG_UPDATE_ERR))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

    async fn delete_model_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_model_config(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(CFG_DELETE_ERR))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

}
