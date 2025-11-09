use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use uuid::Uuid;
use rmcs_resource_db::Resource;
use rmcs_resource_api::slice::slice_service_server::SliceService;
use rmcs_resource_api::slice::{
    SliceSchema, SliceId, SliceTime, SliceRange, SliceNameTime, SliceNameRange, SliceUpdate, SliceOption,
    SliceSetSchema, SliceSetTime, SliceSetRange, SliceSetOption,
    SliceReadResponse, SliceListResponse, SliceCreateResponse, SliceChangeResponse,
    SliceSetReadResponse, SliceSetListResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_SLICE, CREATE_SLICE, UPDATE_SLICE, DELETE_SLICE
};
use crate::utility::handle_error;

#[derive(Debug)]
pub struct SliceServer {
    resource_db: Resource,
    token_key: Vec<u8>,
    accesses: Vec<AccessSchema>
}

impl SliceServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db,
            token_key: Vec::new(),
            accesses: Vec::new()
        }
    }
}

#[tonic::async_trait]
impl SliceService for SliceServer {

    async fn read_slice(&self, request: Request<SliceId>)
        -> Result<Response<SliceReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.read_slice(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceReadResponse { result }))
    }

    async fn list_slice_by_time(&self, request: Request<SliceTime>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_range_time(&self, request: Request<SliceRange>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_range_time(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_name_time(&self, request: Request<SliceNameTime>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_name_time(
            &request.name,
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_name_range_time(&self, request: Request<SliceNameRange>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_name_range_time(
            &request.name,
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_option(&self, request: Request<SliceOption>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_option(
            request.device_id.map(|id| Uuid::from_slice(&id).unwrap_or_default()),
            request.model_id.map(|id| Uuid::from_slice(&id).unwrap_or_default()),
            request.name.as_deref(),
            request.begin.map(|t| Utc.timestamp_nanos(t * 1000)),
            request.end.map(|t| Utc.timestamp_nanos(t * 1000))
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn create_slice(&self, request: Request<SliceSchema>)
        -> Result<Response<SliceCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.create_slice(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp_begin * 1000),
            Utc.timestamp_nanos(request.timestamp_end * 1000),
            &request.name,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceCreateResponse { id }))
    }

    async fn update_slice(&self, request: Request<SliceUpdate>)
        -> Result<Response<SliceChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.update_slice(
            request.id,
            request.timestamp_begin.map(|s| Utc.timestamp_nanos(s * 1000)),
            request.timestamp_end.map(|s| Utc.timestamp_nanos(s * 1000)),
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceChangeResponse { }))
    }

    async fn delete_slice(&self, request: Request<SliceId>)
    -> Result<Response<SliceChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_slice(request.id).await;
        match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceChangeResponse { }))
    }

    async fn read_slice_set(&self, request: Request<SliceId>)
        -> Result<Response<SliceSetReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.read_slice_set(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceSetReadResponse { result }))
    }

    async fn list_slice_set_by_time(&self, request: Request<SliceSetTime>)
        -> Result<Response<SliceSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_set_by_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceSetListResponse { results }))
    }

    async fn list_slice_set_by_range_time(&self, request: Request<SliceSetRange>)
        -> Result<Response<SliceSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_set_by_range_time(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceSetListResponse { results }))
    }

    async fn list_slice_set_by_name_time(&self, request: Request<SliceNameTime>)
        -> Result<Response<SliceSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_set_by_name_time(
            &request.name,
            Utc.timestamp_nanos(request.timestamp * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceSetListResponse { results }))
    }

    async fn list_slice_set_by_name_range_time(&self, request: Request<SliceNameRange>)
        -> Result<Response<SliceSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_set_by_name_range_time(
            &request.name,
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceSetListResponse { results }))
    }

    async fn list_slice_set_option(&self, request: Request<SliceSetOption>)
        -> Result<Response<SliceSetListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_set_option(
            request.set_id.map(|id| Uuid::from_slice(&id).unwrap_or_default()),
            request.name.as_deref(),
            request.begin.map(|t| Utc.timestamp_nanos(t * 1000)),
            request.end.map(|t| Utc.timestamp_nanos(t * 1000))
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceSetListResponse { results }))
    }

    async fn create_slice_set(&self, request: Request<SliceSetSchema>)
        -> Result<Response<SliceCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.create_slice_set(
            Uuid::from_slice(&request.set_id).unwrap_or_default(),
            Utc.timestamp_nanos(request.timestamp_begin * 1000),
            Utc.timestamp_nanos(request.timestamp_end * 1000),
            &request.name,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceCreateResponse { id }))
    }

    async fn update_slice_set(&self, request: Request<SliceUpdate>)
        -> Result<Response<SliceChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.update_slice_set(
            request.id,
            request.timestamp_begin.map(|s| Utc.timestamp_nanos(s * 1000)),
            request.timestamp_end.map(|s| Utc.timestamp_nanos(s * 1000)),
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceChangeResponse { }))
    }

    async fn delete_slice_set(&self, request: Request<SliceId>)
    -> Result<Response<SliceChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_slice_set(request.id).await;
        match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(SliceChangeResponse { }))
    }

}

impl AccessValidator for SliceServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_SLICE, CREATE_SLICE, UPDATE_SLICE, DELETE_SLICE
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
