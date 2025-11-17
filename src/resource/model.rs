use tonic::{Request, Response, Status};
use uuid::Uuid;
use rmcs_resource_db::{Resource, DataType, DataValue};
use rmcs_resource_api::model::model_service_server::ModelService;
use rmcs_resource_api::model::{
    ModelSchema, ModelId, ModelIds, ModelName, ModelCategory, ModelOption, TypeId, ModelUpdate,
    ConfigSchema, ConfigId, ConfigUpdate,
    TagSchema, TagId, TagUpdate,
    ModelReadResponse, ModelListResponse, ModelCreateResponse, ModelChangeResponse,
    ConfigReadResponse, ConfigListResponse, ConfigCreateResponse, ConfigChangeResponse,
    TagReadResponse, TagListResponse, TagChangeResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_MODEL, CREATE_MODEL, UPDATE_MODEL, DELETE_MODEL,
    READ_MODEL_CONFIG, CREATE_MODEL_CONFIG, UPDATE_MODEL_CONFIG, DELETE_MODEL_CONFIG
};
use crate::utility::handle_error;

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
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(ModelReadResponse { result }))
    }

    async fn list_model_by_ids(&self, request: Request<ModelIds>)
        -> Result<Response<ModelListResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_ids(
            request.ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect::<Vec<Uuid>>().as_slice()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(ModelListResponse { results }))
    }

    async fn list_model_by_type(&self, request: Request<TypeId>)
        -> Result<Response<ModelListResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_type(
            Uuid::from_slice(&request.id).unwrap_or_default()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(ModelListResponse { results }))
    }

    async fn list_model_by_name(&self, request: Request<ModelName>)
        -> Result<Response<ModelListResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.list_model_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
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
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(ModelListResponse { results }))
    }

    async fn list_model_option(&self, request: Request<ModelOption>)
        -> Result<Response<ModelListResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.list_model_option(
            request.type_id.map(|id| Uuid::from_slice(&id).unwrap_or_default()),
            request.name.as_deref(),
            request.category.as_deref()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(ModelListResponse { results }))
    }

    async fn create_model(&self, request: Request<ModelSchema>)
        -> Result<Response<ModelCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_MODEL)?;
        let request = request.into_inner();
        let data_type: Vec<DataType> = request.data_type.into_iter().map(|ty| DataType::from(ty)).collect();
        let result = self.resource_db.create_model(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            &data_type,
            &request.category,
            &request.name,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(ModelCreateResponse { id: id.as_bytes().to_vec() }))
    }

    async fn update_model(&self, request: Request<ModelUpdate>)
        -> Result<Response<ModelChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_MODEL)?;
        let request = request.into_inner();
        let data_type: Option<Vec<DataType>> = if request.data_type_flag {
            Some(request.data_type.into_iter().map(|ty| DataType::from(ty)).collect())
        } else {
            None
        };
        let result = self.resource_db.update_model(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            data_type.as_deref(),
            request.category.as_deref(),
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
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
            Err(e) => return Err(handle_error(e))
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
            Err(e) => return Err(handle_error(e))
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
            Err(e) => return Err(handle_error(e))
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
            DataValue::from_bytes(
                &request.config_bytes, 
                DataType::from(request.config_type)
            ),
            &request.category
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
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
                DataValue::from_bytes(
                    &s,
                    DataType::from(request.config_type.unwrap_or_default())
                )
            }),
            request.category.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
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
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

    async fn read_tag(&self, request: Request<TagId>)
        -> Result<Response<TagReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.read_tag(
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            request.tag as i16
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TagReadResponse { result }))
    }

    async fn list_tag_by_model(&self, request: Request<ModelId>)
        -> Result<Response<TagListResponse>, Status>
    {
        self.validate(request.extensions(), READ_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_tag_by_model(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TagListResponse { results }))
    }

    async fn create_tag(&self, request: Request<TagSchema>)
        -> Result<Response<TagChangeResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.create_tag(
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            request.tag as i16,
            &request.name,
            &request.members.into_iter().map(|t| t as i16).collect::<Vec<i16>>()
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TagChangeResponse { }))
    }

    async fn update_tag(&self, request: Request<TagUpdate>)
        -> Result<Response<TagChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_MODEL_CONFIG)?;
        let request = request.into_inner();
        let members = if request.members_flag {
            Some(request.members.into_iter().map(|t| t as i16).collect::<Vec<i16>>())
        } else {
            None
        };
        let result = self.resource_db.update_tag(
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            request.tag as i16,
            request.name.as_deref(),
            members.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TagChangeResponse { }))
    }

    async fn delete_tag(&self, request: Request<TagId>)
        -> Result<Response<TagChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_MODEL_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_tag(
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            request.tag as i16
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TagChangeResponse { }))
    }

}

impl AccessValidator for ModelServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_MODEL, CREATE_MODEL, UPDATE_MODEL, DELETE_MODEL,
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
