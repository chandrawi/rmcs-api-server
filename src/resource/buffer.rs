use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_resource_db::{Resource, DataType, ArrayDataValue};
use rmcs_resource_api::buffer::buffer_service_server::BufferService;
use rmcs_resource_api::common;
use rmcs_resource_api::buffer::{
    BufferSchema, BufferId, BufferSelector, BuffersSelector, BufferUpdate,
    BufferReadResponse, BufferListResponse, BufferCreateResponse, BufferChangeResponse,
    BufferStatus
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_BUFFER, READ_BUFFER_FIRST, READ_BUFFER_LAST, LIST_BUFFER_FIRST, LIST_BUFFER_LAST,
    CREATE_BUFFER, UPDATE_BUFFER, DELETE_BUFFER
};
use super::{
    BUFFER_NOT_FOUND, BUFFER_CREATE_ERR, BUFFER_UPDATE_ERR, BUFFER_DELETE_ERR
};

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
            Err(_) => return Err(Status::not_found(BUFFER_NOT_FOUND))
        };
        Ok(Response::new(BufferReadResponse { result }))
    }

    async fn read_buffer_first(&self, request: Request<BufferSelector>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER_FIRST)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_first(
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(BUFFER_NOT_FOUND))
        };
        Ok(Response::new(BufferReadResponse { result }))
    }

    async fn read_buffer_last(&self, request: Request<BufferSelector>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_BUFFER_LAST)?;
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_last(
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(BUFFER_NOT_FOUND))
        };
        Ok(Response::new(BufferReadResponse { result }))
    }

    async fn list_buffer_first(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_BUFFER_FIRST)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first(
            request.number,
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(BUFFER_NOT_FOUND))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn list_buffer_last(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_BUFFER_LAST)?;
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last(
            request.number,
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(BUFFER_NOT_FOUND))
        };
        Ok(Response::new(BufferListResponse { results }))
    }

    async fn create_buffer(&self, request: Request<BufferSchema>)
        -> Result<Response<BufferCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_BUFFER)?;
        let request = request.into_inner();
        let result = self.resource_db.create_buffer(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp),
            Some(request.index as u16),
            ArrayDataValue::from_bytes(
                &request.data_bytes,
                request.data_type.into_iter().map(|e| {
                    DataType::from(common::DataType::from_i32(e).unwrap_or_default())
                }).collect::<Vec<DataType>>().as_slice()
            ).to_vec(),
            BufferStatus::from_i32(request.status).unwrap_or_default().as_str_name()
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(BUFFER_CREATE_ERR))
        };
        Ok(Response::new(BufferCreateResponse { id }))
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
                    request.data_type.into_iter().map(|e| {
                        DataType::from(common::DataType::from_i32(e).unwrap_or_default())
                    }).collect::<Vec<DataType>>().as_slice()
                ).to_vec()
            }),
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(BUFFER_UPDATE_ERR))
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
            Err(_) => return Err(Status::internal(BUFFER_DELETE_ERR))
        };
        Ok(Response::new(BufferChangeResponse { }))
    }

}

impl AccessValidator for BufferServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_BUFFER, READ_BUFFER_FIRST, READ_BUFFER_LAST, LIST_BUFFER_FIRST, LIST_BUFFER_LAST,
            CREATE_BUFFER, UPDATE_BUFFER, DELETE_BUFFER
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
