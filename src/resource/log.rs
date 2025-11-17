use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use uuid::Uuid;
use rmcs_resource_db::{Resource, DataType, DataValue};
use rmcs_resource_api::log::log_service_server::LogService;
use rmcs_resource_api::log::{
    LogSchema, LogId, LogIds, LogTime, LogRange, LogUpdate, LogUpdateTime,
    LogReadResponse, LogListResponse, LogCreateResponse, LogChangeResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_LOG, CREATE_LOG, UPDATE_LOG, DELETE_LOG
};
use crate::utility::handle_error;

#[derive(Debug)]
pub struct LogServer {
    resource_db: Resource,
    token_key: Vec<u8>,
    accesses: Vec<AccessSchema>
}

impl LogServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db,
            token_key: Vec::new(),
            accesses: Vec::new()
        }
    }
}

#[tonic::async_trait]
impl LogService for LogServer {

    async fn read_log(&self, request: Request<LogId>)
        -> Result<Response<LogReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.read_log(
            request.id
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogReadResponse { result }))
    }

    async fn read_log_by_time(&self, request: Request<LogTime>)
        -> Result<Response<LogReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.read_log_by_time(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogReadResponse { result }))
    }

    async fn list_log_by_ids(&self, request: Request<LogIds>)
        -> Result<Response<LogListResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_ids(
            &request.ids
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogListResponse { results }))
    }

    async fn list_log_by_time(&self, request: Request<LogTime>)
        -> Result<Response<LogListResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_time(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogListResponse { results }))
    }

    async fn list_log_by_latest(&self, request: Request<LogTime>)
        -> Result<Response<LogListResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_latest(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogListResponse { results }))
    }

    async fn list_log_by_range(&self, request: Request<LogRange>)
        -> Result<Response<LogListResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_range(
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogListResponse { results }))
    }

    async fn create_log(&self, request: Request<LogSchema>)
        -> Result<Response<LogCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.create_log(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            DataValue::from_bytes(
                &request.log_bytes, 
                DataType::from(request.log_type)
            ),
            Some(request.tag as i16)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogCreateResponse { id }))
    }

    async fn update_log(&self, request: Request<LogUpdate>)
    -> Result<Response<LogChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.update_log(
            request.id,
            request.log_bytes.map(|s| {
                DataValue::from_bytes(
                    &s, 
                    DataType::from(request.log_type.unwrap_or_default())
                )
            }),
            request.tag.map(|t| t as i16)
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogChangeResponse { }))
    }

    async fn update_log_by_time(&self, request: Request<LogUpdateTime>)
    -> Result<Response<LogChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.update_log_by_time(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.log_bytes.map(|s| {
                DataValue::from_bytes(
                    &s, 
                    DataType::from(request.log_type.unwrap_or_default())
                )
            }),
            request.tag.map(|t| t as i16)
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogChangeResponse { }))
    }

    async fn delete_log(&self, request: Request<LogId>)
        -> Result<Response<LogChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_log(
            request.id
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogChangeResponse { }))
    }

    async fn delete_log_by_time(&self, request: Request<LogTime>)
        -> Result<Response<LogChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_log_by_time(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.model_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.tag.map(|t| t as i16)
        ).await;
        match result {
            Ok(_) => (),
            Err(e) => return Err(handle_error(e))
        };
        Ok(Response::new(LogChangeResponse { }))
    }

}

impl AccessValidator for LogServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_LOG, CREATE_LOG, UPDATE_LOG, DELETE_LOG
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
