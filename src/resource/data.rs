use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use uuid::Uuid;
use rmcs_resource_db::{Resource, DataType, ArrayDataValue};
use rmcs_resource_api::data::data_service_server::DataService;
use rmcs_resource_api::common;
use rmcs_resource_api::data::{
    DataSchema, DataId, DataTime, DataRange, DataNumber,
    ModelId, DataIdModel, DataTimeModel, DataRangeModel, DataNumberModel,
    DataReadResponse, DataListResponse, DataModelResponse, DataChangeResponse, DataSchemaModel
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_DATA, LIST_DATA_BY_TIME, LIST_DATA_BY_LAST_TIME, LIST_DATA_BY_RANGE_TIME,
    LIST_DATA_BY_NUMBER_BEFORE, LIST_DATA_BY_NUMBER_AFTER, GET_DATA_MODEL,
    READ_DATA_WITH_MODEL, LIST_DATA_WITH_MODEL_BY_TIME, LIST_DATA_WITH_MODEL_BY_LAST_TIME,
    LIST_DATA_WITH_MODEL_BY_RANGE_TIME, LIST_DATA_WITH_MODEL_BY_NUMBER_BEFORE, LIST_DATA_WITH_MODEL_BY_NUMBER_AFTER,
    CREATE_DATA, CREATE_DATA_WITH_MODEL, DELETE_DATA, DELETE_DATA_WITH_MODEL
};
use super::{
    DATA_NOT_FOUND, DATA_CREATE_ERR, DATA_DELETE_ERR, DATA_MODEL_NOT_FOUND
};

#[derive(Debug)]
pub struct DataServer {
    resource_db: Resource,
    token_key: Vec<u8>,
    accesses: Vec<AccessSchema>
}

impl DataServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db,
            token_key: Vec::new(),
            accesses: Vec::new()
        }
    }
}

#[tonic::async_trait]
impl DataService for DataServer {

    async fn read_data(&self, request: Request<DataId>)
        -> Result<Response<DataReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.read_data(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            Some(request.index)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataReadResponse { result }))
    }

    async fn list_data_by_time(&self, request: Request<DataTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_BY_TIME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_last_time(&self, request: Request<DataTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_BY_LAST_TIME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_last_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_range_time(&self, request: Request<DataRange>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_BY_RANGE_TIME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_range_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_number_before(&self, request: Request<DataNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_BY_NUMBER_BEFORE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_number_before(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_number_after(&self, request: Request<DataNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_BY_NUMBER_AFTER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_number_after(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn get_data_model(&self, request: Request<ModelId>)
        -> Result<Response<DataModelResponse>, Status>
    {
        self.validate(request.extensions(), GET_DATA_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.get_data_model(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(DATA_MODEL_NOT_FOUND))
        };
        Ok(Response::new(DataModelResponse { result }))
    }

    async fn read_data_with_model(&self, request: Request<DataIdModel>)
        -> Result<Response<DataReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA_WITH_MODEL)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataReadResponse { result: None }));
        }
        let result = self.resource_db.read_data_with_model(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            Some(request.index)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataReadResponse { result }))
    }

    async fn list_data_with_model_by_time(&self, request: Request<DataTimeModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_WITH_MODEL_BY_TIME)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new() }));
        }
        let result = self.resource_db.list_data_with_model_by_time(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_with_model_by_last_time(&self, request: Request<DataTimeModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_WITH_MODEL_BY_LAST_TIME)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new() }));
        }
        let result = self.resource_db.list_data_with_model_by_last_time(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_with_model_by_range_time(&self, request: Request<DataRangeModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_WITH_MODEL_BY_RANGE_TIME)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new() }));
        }
        let result = self.resource_db.list_data_with_model_by_range_time(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_with_model_by_number_before(&self, request: Request<DataNumberModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_WITH_MODEL_BY_NUMBER_BEFORE)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new() }));
        }
        let result = self.resource_db.list_data_with_model_by_number_before(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_with_model_by_number_after(&self, request: Request<DataNumberModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DATA_WITH_MODEL_BY_NUMBER_AFTER)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new() }));
        }
        let result = self.resource_db.list_data_with_model_by_number_after(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn create_data(&self, request: Request<DataSchema>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.create_data(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            Some(request.index),
            ArrayDataValue::from_bytes(
                &request.data_bytes,
                request.data_type.into_iter().map(|e| {
                    DataType::from(common::DataType::from_i32(e).unwrap_or_default())
                }).collect::<Vec<DataType>>().as_slice()
            ).to_vec()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DATA_CREATE_ERR))
        };
        Ok(Response::new(DataChangeResponse { }))
    }

    async fn create_data_with_model(&self, request: Request<DataSchemaModel>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_DATA_WITH_MODEL)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataChangeResponse { }));
        }
        let result = self.resource_db.create_data_with_model(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            Some(request.index),
            ArrayDataValue::from_bytes(
                &request.data_bytes,
                request.data_type.into_iter().map(|e| {
                    DataType::from(common::DataType::from_i32(e).unwrap_or_default())
                }).collect::<Vec<DataType>>().as_slice()
            ).to_vec()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DATA_CREATE_ERR))
        };
        Ok(Response::new(DataChangeResponse { }))
    }

    async fn delete_data(&self, request: Request<DataId>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_data(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            Some(request.index)
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataChangeResponse { }))
    }

    async fn delete_data_with_model(&self, request: Request<DataIdModel>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_DATA_WITH_MODEL)?;
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataChangeResponse { }));
        }
        let result = self.resource_db.delete_data_with_model(
            request.model.unwrap().into(),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            Some(request.index)
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataChangeResponse { }))
    }

}

impl AccessValidator for DataServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_DATA, LIST_DATA_BY_TIME, LIST_DATA_BY_LAST_TIME, LIST_DATA_BY_RANGE_TIME,
            LIST_DATA_BY_NUMBER_BEFORE, LIST_DATA_BY_NUMBER_AFTER, GET_DATA_MODEL,
            READ_DATA_WITH_MODEL, LIST_DATA_WITH_MODEL_BY_TIME, LIST_DATA_WITH_MODEL_BY_LAST_TIME,
            LIST_DATA_WITH_MODEL_BY_RANGE_TIME, LIST_DATA_WITH_MODEL_BY_NUMBER_BEFORE, LIST_DATA_WITH_MODEL_BY_NUMBER_AFTER,
            CREATE_DATA, CREATE_DATA_WITH_MODEL, DELETE_DATA, DELETE_DATA_WITH_MODEL
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
