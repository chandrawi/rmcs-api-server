use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use uuid::Uuid;
use rmcs_resource_db::{Resource, DataType, ArrayDataValue};
use rmcs_resource_api::data::data_service_server::DataService;
use rmcs_resource_api::common;
use rmcs_resource_api::data::{
    DataSchema, DataId, DataTime, DataRange, DataNumber, DataIds, DataIdsTime, DataIdsRange, DataIdsNumber,
    DataSetId, DataSetTime, DataSetRange, DataSetNumber,
    DataReadResponse, DataListResponse, DataChangeResponse, DataSetReadResponse, DataSetListResponse, DataCountResponse,
    TimestampReadResponse, TimestampListResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_DATA, CREATE_DATA, DELETE_DATA
};
use super::{
    DATA_NOT_FOUND, DATA_CREATE_ERR, DATA_DELETE_ERR
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
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataReadResponse { result }))
    }

    async fn list_data_by_last_time(&self, request: Request<DataTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
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
        self.validate(request.extensions(), READ_DATA)?;
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
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_number_before(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
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
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_number_after(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_ids_time(&self, request: Request<DataIdsTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_ids_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_ids_last_time(&self, request: Request<DataIdsTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_ids_last_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_ids_range_time(&self, request: Request<DataIdsRange>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_ids_range_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_ids_number_before(&self, request: Request<DataIdsNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_ids_number_before(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_ids_number_after(&self, request: Request<DataIdsNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_ids_number_after(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_set_time(&self, request: Request<DataSetTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_set_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_set_last_time(&self, request: Request<DataSetTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_set_last_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_set_range_time(&self, request: Request<DataSetRange>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_set_range_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_set_number_before(&self, request: Request<DataSetNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_set_number_before(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn list_data_by_set_number_after(&self, request: Request<DataSetNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_set_number_after(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataListResponse { results }))
    }

    async fn read_data_set(&self, request: Request<DataSetId>)
        -> Result<Response<DataSetReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.read_data_set(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataSetReadResponse { result }))
    }

    async fn list_data_set_by_last_time(&self, request: Request<DataSetTime>)
        -> Result<Response<DataSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_set_by_last_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataSetListResponse { results }))
    }

    async fn list_data_set_by_range_time(&self, request: Request<DataSetRange>)
        -> Result<Response<DataSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_set_by_range_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataSetListResponse { results }))
    }

    async fn list_data_set_by_number_before(&self, request: Request<DataSetNumber>)
        -> Result<Response<DataSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_set_by_number_before(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataSetListResponse { results }))
    }

    async fn list_data_set_by_number_after(&self, request: Request<DataSetNumber>)
        -> Result<Response<DataSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_set_by_number_after(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(DataSetListResponse { results }))
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
            ArrayDataValue::from_bytes(
                &request.data_bytes,
                request.data_type.into_iter().map(|e| {
                    DataType::from(common::DataType::try_from(e).unwrap_or_default())
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
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataChangeResponse { }))
    }

    async fn read_data_timestamp(&self, request: Request<DataId>)
        -> Result<Response<TimestampReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.read_data_timestamp(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let timestamp = match result {
            Ok(value) => value.timestamp_micros(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampReadResponse { timestamp }))
    }

    async fn list_data_timestamp_by_last_time(&self, request: Request<DataTime>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_timestamp_by_last_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_data_timestamp_by_range_time(&self, request: Request<DataRange>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_timestamp_by_range_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn read_data_timestamp_by_ids(&self, request: Request<DataIds>)
        -> Result<Response<TimestampReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.read_data_timestamp_by_ids(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let timestamp = match result {
            Ok(value) => value.timestamp_micros(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampReadResponse { timestamp }))
    }

    async fn list_data_timestamp_by_ids_last_time(&self, request: Request<DataIdsTime>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_timestamp_by_ids_last_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_data_timestamp_by_ids_range_time(&self, request: Request<DataIdsRange>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_timestamp_by_ids_range_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn read_data_timestamp_by_set(&self, request: Request<DataSetId>)
        -> Result<Response<TimestampReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.read_data_timestamp_by_set(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let timestamp = match result {
            Ok(value) => value.timestamp_micros(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampReadResponse { timestamp }))
    }

    async fn list_data_timestamp_by_set_last_time(&self, request: Request<DataSetTime>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_timestamp_by_set_last_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_data_timestamp_by_set_range_time(&self, request: Request<DataSetRange>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.list_data_timestamp_by_set_range_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(_) => return Err(Status::not_found(DATA_NOT_FOUND))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn count_data(&self, request: Request<DataTime>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default()
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_last_time(&self, request: Request<DataTime>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_last_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_range_time(&self, request: Request<DataRange>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_range_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_ids(&self, request: Request<DataIdsTime>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_ids(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_ids_last_time(&self, request: Request<DataIdsTime>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_ids_last_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_ids_range_time(&self, request: Request<DataIdsRange>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_ids_range_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_set(&self, request: Request<DataSetTime>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_set(
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_set_last_time(&self, request: Request<DataSetTime>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_set_last_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

    async fn count_data_by_set_range_time(&self, request: Request<DataSetRange>)
        -> Result<Response<DataCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_DATA)?;
        let request = request.into_inner();
        let result = self.resource_db.count_data_by_set_range_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(_) => return Err(Status::internal(DATA_DELETE_ERR))
        };
        Ok(Response::new(DataCountResponse { count }))
    }

}

impl AccessValidator for DataServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_DATA, CREATE_DATA, DELETE_DATA
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
