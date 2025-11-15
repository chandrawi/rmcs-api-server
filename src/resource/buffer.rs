use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use uuid::Uuid;
use rmcs_resource_db::{Resource, DataType, ArrayDataValue};
use rmcs_resource_api::buffer::buffer_service_server::BufferService;
use rmcs_resource_api::buffer::{
    BufferSchema, BufferMultipleSchema, BufferId, BufferIds, BufferTime, BufferRange, BufferNumber, 
    BufferSelector, BuffersSelector, BufferUpdate, BufferUpdateTime,
    BufferIdsTime, BufferIdsRange, BufferIdsNumber, BufferIdsSelector, BuffersIdsSelector,
    BufferSetTime, BufferSetRange, BufferSetNumber, BufferSetSelector, BuffersSetSelector,
    BufferReadResponse, BufferListResponse, BufferCreateResponse, BufferCreateMultipleResponse, BufferChangeResponse, BufferCountResponse,
    TimestampReadResponse, TimestampListResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_BUFFER, CREATE_BUFFER, UPDATE_BUFFER, DELETE_BUFFER
};
use crate::utility::handle_error;

#[derive(Debug)]
pub struct BufferServer {
    resource_db: Resource,
    token_key: Vec<u8>,
    accesses: Vec<AccessSchema>
}

impl BufferServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db,
            token_key: Vec::new(),
            accesses: Vec::new()
        }
    }
}

#[tonic::async_trait]
impl BufferService for BufferServer {

    async fn read_buffer(&self, request: Request<BufferId>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferReadResponse { result }))
    }

    async fn read_buffer_by_time(&self, request: Request<BufferTime>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_by_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferReadResponse { result }))
    }

    async fn list_buffer(&self, request: Request<BufferIds>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer(request.ids).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_time(&self, request: Request<BufferTime>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_last_time(&self, request: Request<BufferTime>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_last_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_range_time(&self, request: Request<BufferRange>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_range_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_number_before(&self, request: Request<BufferNumber>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_number_before(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize,
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_number_after(&self, request: Request<BufferNumber>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_number_after(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize,
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn read_buffer_first(&self, request: Request<BufferSelector>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_first(
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferReadResponse { result }))
    }

    async fn read_buffer_last(&self, request: Request<BufferSelector>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_last(
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferReadResponse { result }))
    }

    async fn list_buffer_first(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first(
            request.number as usize,
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_first_offset(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first_offset(
            request.number as usize,
            request.offset as usize,
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_last(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last(
            request.number as usize,
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_last_offset(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last_offset(
            request.number as usize,
            request.offset as usize,
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_ids_time(&self, request: Request<BufferIdsTime>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_ids_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_ids_last_time(&self, request: Request<BufferIdsTime>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_ids_last_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_ids_range_time(&self, request: Request<BufferIdsRange>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_ids_range_time(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_ids_number_before(&self, request: Request<BufferIdsNumber>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_ids_number_before(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize,
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_ids_number_after(&self, request: Request<BufferIdsNumber>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_ids_number_after(
            request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize,
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_first_by_ids(&self, request: Request<BuffersIdsSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first_by_ids(
            request.number as usize,
            Some(request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            Some(request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_first_offset_by_ids(&self, request: Request<BuffersIdsSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first_offset_by_ids(
            request.number as usize,
            request.offset as usize,
            Some(request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            Some(request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_last_by_ids(&self, request: Request<BuffersIdsSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last_by_ids(
            request.number as usize,
            Some(request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            Some(request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_last_offset_by_ids(&self, request: Request<BuffersIdsSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last_offset_by_ids(
            request.number as usize,
            request.offset as usize,
            Some(request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            Some(request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_set_time(&self, request: Request<BufferSetTime>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_set_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_set_last_time(&self, request: Request<BufferSetTime>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_set_last_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_set_range_time(&self, request: Request<BufferSetRange>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_set_range_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_set_number_before(&self, request: Request<BufferSetNumber>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_set_number_before(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_by_set_number_after(&self, request: Request<BufferSetNumber>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_by_set_number_after(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.number as usize
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_first_by_set(&self, request: Request<BuffersSetSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first_by_set(
            request.number as usize,
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_first_offset_by_set(&self, request: Request<BuffersSetSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first_offset_by_set(
            request.number as usize,
            request.offset as usize,
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_last_by_set(&self, request: Request<BuffersSetSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last_by_set(
            request.number as usize,
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_last_offset_by_set(&self, request: Request<BuffersSetSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last_offset_by_set(
            request.number as usize,
            request.offset as usize,
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn create_buffer(&self, request: Request<BufferSchema>)
        -> Result<Response<BufferCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.create_buffer(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            ArrayDataValue::from_bytes(
                &request.data_bytes,
                request.data_type.into_iter().map(|e| DataType::from(e)).collect::<Vec<DataType>>().as_slice()
            ).to_vec(),
            Some(request.tag as i16)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferCreateResponse { id }))
    }

    async fn create_buffer_multiple(&self, request: Request<BufferMultipleSchema>)
        -> Result<Response<BufferCreateMultipleResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_BUFFER)?;
        let request = request.into_inner();
        let (device_ids, model_ids, timestamps, data_multiple, tags) = request.schemas.into_iter().map(|r| {(
            Uuid::from_slice(&r.device_id).unwrap_or_default(),
            Uuid::from_slice(&r.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(&r.timestamp * 1000),
            ArrayDataValue::from_bytes(
                &r.data_bytes,
                &r.data_type.iter().map(|&e| DataType::from(e)).collect::<Vec<DataType>>().as_slice()
            ).to_vec(),
            r.tag as i16
        )}).collect();
        let result = self.resource_db.create_buffer_multiple(
            device_ids,
            model_ids,
            timestamps,
            data_multiple,
            Some(tags)
        ).await;
        let ids = match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferCreateMultipleResponse { ids }))
    }

    async fn update_buffer(&self, request: Request<BufferUpdate>)
        -> Result<Response<BufferChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.update_buffer(
            request.id,
            request.data_bytes.map(|s| {
                ArrayDataValue::from_bytes(
                    &s,
                    request.data_type.into_iter().map(|e| DataType::from(e)).collect::<Vec<DataType>>().as_slice()
                ).to_vec()
            }),
            request.tag.map(|t| t as i16)
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferChangeResponse { }))
    }

    async fn update_buffer_by_time(&self, request: Request<BufferUpdateTime>)
        -> Result<Response<BufferChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.update_buffer_by_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.data_bytes.map(|s| {
                ArrayDataValue::from_bytes(
                    &s,
                    request.data_type.into_iter().map(|e| DataType::from(e)).collect::<Vec<DataType>>().as_slice()
                ).to_vec()
            }),
            request.tag.map(|t| t as i16)
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferChangeResponse { }))
    }

    async fn delete_buffer(&self, request: Request<BufferId>)
        -> Result<Response<BufferChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_buffer(request.id).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferChangeResponse { }))
    }

    async fn delete_buffer_by_time(&self, request: Request<BufferTime>)
        -> Result<Response<BufferChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_buffer_by_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.tag.map(|t| t as i16)
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferChangeResponse { }))
    }

    async fn read_buffer_timestamp_first(&self, request: Request<BufferSelector>)
        -> Result<Response<TimestampReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_timestamp_first(
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let timestamp = match result {
            Ok(value) => value.timestamp_micros(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampReadResponse { timestamp }))
    }

    async fn read_buffer_timestamp_last(&self, request: Request<BufferSelector>)
        -> Result<Response<TimestampReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_timestamp_last(
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let timestamp = match result {
            Ok(value) => value.timestamp_micros(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampReadResponse { timestamp }))
    }

    async fn list_buffer_timestamp_first(&self, request: Request<BuffersSelector>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_timestamp_first(
            request.number as usize,
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_buffer_timestamp_last(&self, request: Request<BuffersSelector>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_timestamp_last(
            request.number as usize,
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_buffer_timestamp_first_by_ids(&self, request: Request<BuffersIdsSelector>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_timestamp_first_by_ids(
            request.number as usize,
            Some(request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            Some(request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            request.tag.map(|t| t as i16)
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_buffer_timestamp_last_by_ids(&self, request: Request<BuffersIdsSelector>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_timestamp_last_by_ids(
            request.number as usize,
            Some(request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            Some(request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            request.tag.map(|t| t as i16)
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_buffer_timestamp_first_by_set(&self, request: Request<BuffersSetSelector>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_timestamp_first_by_set(
            request.number as usize,
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn list_buffer_timestamp_last_by_set(&self, request: Request<BuffersSetSelector>)
        -> Result<Response<TimestampListResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_timestamp_last_by_set(
            request.number as usize,
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let timestamps = match result {
            Ok(value) => value.into_iter().map(|t| t.timestamp_micros()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(TimestampListResponse { timestamps }))
    }

    async fn count_buffer(&self, request: Request<BufferSelector>)
        -> Result<Response<BufferCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.count_buffer(
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferCountResponse { count }))
    }

    async fn count_buffer_by_ids(&self, request: Request<BufferIdsSelector>)
        -> Result<Response<BufferCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.count_buffer_by_ids(
            Some(request.device_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            Some(request.model_ids.into_iter().map(|id| Uuid::from_slice(&id).unwrap_or_default()).collect()),
            request.tag.map(|t| t as i16)
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferCountResponse { count }))
    }

    async fn count_buffer_by_set(&self, request: Request<BufferSetSelector>)
        -> Result<Response<BufferCountResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.count_buffer_by_set(
            Uuid::from_slice(&request.set_id).unwrap_or_default()
        ).await;
        let count = match result {
            Ok(value) => value as u32,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(BufferCountResponse { count }))
    }

}

impl AccessValidator for BufferServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_BUFFER, CREATE_BUFFER, UPDATE_BUFFER, DELETE_BUFFER
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
