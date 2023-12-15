use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use uuid::Uuid;
use rmcs_resource_db::Resource;
use rmcs_resource_api::slice::slice_service_server::SliceService;
use rmcs_resource_api::slice::{
    SliceSchema, SliceId, SliceName, SliceDevice, SliceModel, SliceDeviceModel, SliceUpdate,
    SliceReadResponse, SliceListResponse, SliceCreateResponse, SliceChangeResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_SLICE, CREATE_SLICE, UPDATE_SLICE, DELETE_SLICE
};
use super::{
    SLICE_NOT_FOUND, SLICE_CREATE_ERR, SLICE_UPDATE_ERR, SLICE_DELETE_ERR
};

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
            Err(_) => return Err(Status::not_found(SLICE_NOT_FOUND))
        };
        Ok(Response::new(SliceReadResponse { result }))
    }

    async fn list_slice_by_name(&self, request: Request<SliceName>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(SLICE_NOT_FOUND))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_device(&self, request: Request<SliceDevice>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_device(Uuid::from_slice(&request.device_id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(SLICE_NOT_FOUND))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_model(&self, request: Request<SliceModel>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_model(Uuid::from_slice(&request.model_id).unwrap_or_default()).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(SLICE_NOT_FOUND))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_device_model(&self, request: Request<SliceDeviceModel>)
        -> Result<Response<SliceListResponse>, Status>
    {
        self.validate(request.extensions(), READ_SLICE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_device_model(
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default()
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(SLICE_NOT_FOUND))
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
            Err(_) => return Err(Status::internal(SLICE_CREATE_ERR))
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
            Err(_) => return Err(Status::internal(SLICE_UPDATE_ERR))
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
            Err(_) => return Err(Status::internal(SLICE_DELETE_ERR))
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
