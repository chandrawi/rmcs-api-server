use tonic::{Request, Response, Status};
use uuid::Uuid;
use rmcs_resource_db::{Resource, DataType, ConfigType, ConfigValue};
use rmcs_resource_api::model::model_service_server::ModelService;
use rmcs_resource_api::common;
use rmcs_resource_api::model::{
    ModelSchema, ModelId, ModelName, ModelCategory, ModelNameCategory, ModelUpdate, ModelTypes,
    ConfigSchema, ConfigId, ConfigUpdate,
    ModelReadResponse, ModelListResponse, ModelCreateResponse, ModelChangeResponse,
    ConfigReadResponse, ConfigListResponse, ConfigCreateResponse, ConfigChangeResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_MODEL, CREATE_MODEL, UPDATE_MODEL, DELETE_MODEL, CHANGE_MODEL_TYPE,
    READ_MODEL_CONFIG, CREATE_MODEL_CONFIG, UPDATE_MODEL_CONFIG, DELETE_MODEL_CONFIG
};
use super::{
    MODEL_NOT_FOUND, MODEL_CREATE_ERR, MODEL_UPDATE_ERR, MODEL_DELETE_ERR, ADD_TYPE_ERR, RMV_TYPE_ERR,
    CFG_NOT_FOUND, CFG_CREATE_ERR, CFG_UPDATE_ERR, CFG_DELETE_ERR
};

#[derive(Debug)]
pub struct ModelServer {
    resource_db: Resource,
    token_key: Vec<u8>,
    accesses: Vec<AccessSchema>
}

impl ModelServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db,
            token_key: Vec::new(),
            accesses: Vec::new()
        }
    }
}

#[tonic::async_trait]
impl ModelService for ModelServer {

    async fn read_model(&self, request: Request<ModelId>)
        -> Result<Response<ModelReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.read_model(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(MODEL_NOT_FOUND))
        };
        Ok(Response::new(ModelReadResponse { result }))
    }

    async fn list_model_by_name(&self, request: Request<ModelName>)
        -> Result<Response<ModelListResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL)?;
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
        self.validate(request.extensions(), READ_MODEL)?;
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
        self.validate(request.extensions(), READ_MODEL)?;
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
        self.validate(request.extensions(), CREATE_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.create_model(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            &request.category,
            &request.name,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(MODEL_CREATE_ERR))
        };
        Ok(Response::new(ModelCreateResponse { id: id.as_bytes().to_vec() }))
    }

    async fn update_model(&self, request: Request<ModelUpdate>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.update_model(
            Uuid::from_slice(&request.id).unwrap_or_default(),
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
        self.validate(request.extensions(), DELETE_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_model(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(MODEL_DELETE_ERR))
        };
        Ok(Response::new(ModelChangeResponse { }))
    }

    async fn add_model_type(&self, request: Request<ModelTypes>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        self.validate(request.extensions(), CHANGE_MODEL_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.add_model_type(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            request.types.into_iter().map(|e| {
                DataType::from(common::DataType::try_from(e).unwrap_or_default())
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
        self.validate(request.extensions(), CHANGE_MODEL_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.remove_model_type(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_TYPE_ERR))
        };
        Ok(Response::new(ModelChangeResponse { }))
    }

    async fn read_model_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL_CONFIG)?;
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
        self.validate(request.extensions(), READ_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_model_config_by_model(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(CFG_NOT_FOUND))
        };
        Ok(Response::new(ConfigListResponse { results }))
    }

    async fn create_model_config(&self, request: Request<ConfigSchema>)
        -> Result<Response<ConfigCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.create_model_config(
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            request.index,
            &request.name,
            ConfigValue::from_bytes(
                &request.config_bytes, 
                ConfigType::from(common::ConfigType::try_from(request.config_type).unwrap_or_default())
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
        self.validate(request.extensions(), UPDATE_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.update_model_config(
            request.id,
            request.name.as_deref(),
            request.config_bytes.map(|s| {
                ConfigValue::from_bytes(
                    &s,
                    ConfigType::from(common::ConfigType::try_from(request.config_type.unwrap_or_default()).unwrap_or_default())
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
        self.validate(request.extensions(), DELETE_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_model_config(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(CFG_DELETE_ERR))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

}

impl AccessValidator for ModelServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_MODEL, CREATE_MODEL, UPDATE_MODEL, DELETE_MODEL, CHANGE_MODEL_TYPE,
            READ_MODEL_CONFIG, CREATE_MODEL_CONFIG, UPDATE_MODEL_CONFIG, DELETE_MODEL_CONFIG
        ];
        self.token_key = token_key.to_owned();
        self.accesses = Self::construct_accesses(accesses, PROCEDURES);
        self
    }

    fn token_key(&self) -> Vec<u8> {
        self.token_key.clone()
    }

    fn accesses(&self) -> Vec<AccessSchema> {
        self.accesses.clone()
    }

}
