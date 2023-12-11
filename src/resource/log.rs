use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use uuid::Uuid;
use rmcs_resource_db::{Resource, LogType, LogValue};
use rmcs_resource_api::log::log_service_server::LogService;
use rmcs_resource_api::common;
use rmcs_resource_api::log::{
    LogSchema, LogId, LogTime, LogRange, LogUpdate,
    LogReadResponse, LogListResponse, LogChangeResponse,
    LogStatus
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_LOG, CREATE_LOG, UPDATE_LOG, DELETE_LOG
};
use super::{
    LOG_NOT_FOUND, LOG_CREATE_ERR, LOG_UPDATE_ERR, LOG_DELETE_ERR
};

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
            Utc.timestamp_nanos(request.timestamp * 1000),
            Uuid::from_slice(&request.device_id).unwrap_or_default()
        ).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(LOG_NOT_FOUND))
        };
        Ok(Response::new(LogReadResponse { result }))
    }

    async fn list_log_by_time(&self, request: Request<LogTime>)
        -> Result<Response<LogListResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_time(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(LOG_NOT_FOUND))
        };
        Ok(Response::new(LogListResponse { results }))
    }

    async fn list_log_by_last_time(&self, request: Request<LogTime>)
        -> Result<Response<LogListResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_last_time(
            Utc.timestamp_nanos(request.timestamp * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(LOG_NOT_FOUND))
        };
        Ok(Response::new(LogListResponse { results }))
    }

    async fn list_log_by_range_time(&self, request: Request<LogRange>)
        -> Result<Response<LogListResponse>, Status>
    {
        self.validate(request.extensions(), READ_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_range_time(
            Utc.timestamp_nanos(request.begin * 1000),
            Utc.timestamp_nanos(request.end * 1000),
            request.device_id.map(|x| Uuid::from_slice(&x).unwrap_or_default()),
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(LOG_NOT_FOUND))
        };
        Ok(Response::new(LogListResponse { results }))
    }

    async fn create_log(&self, request: Request<LogSchema>)
        -> Result<Response<LogChangeResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.create_log(
            Utc.timestamp_nanos(request.timestamp * 1000),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            LogStatus::from_i32(request.status).unwrap_or_default().as_str_name(),
            LogValue::from_bytes(
                &request.log_bytes, 
                LogType::from(common::ConfigType::from_i32(request.log_type).unwrap_or_default())
            )
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(LOG_CREATE_ERR))
        };
        Ok(Response::new(LogChangeResponse { }))
    }

    async fn update_log(&self, request: Request<LogUpdate>)
    -> Result<Response<LogChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.update_log(
            Utc.timestamp_nanos(request.timestamp * 1000),
            Uuid::from_slice(&request.device_id).unwrap_or_default(),
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name()),
            request.log_bytes.map(|s| {
                LogValue::from_bytes(
                    &s, 
                    LogType::from(common::ConfigType::from_i32(request.log_type.unwrap_or_default()).unwrap_or_default())
                )
            })
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(LOG_UPDATE_ERR))
        };
        Ok(Response::new(LogChangeResponse { }))
    }

    async fn delete_log(&self, request: Request<LogId>)
        -> Result<Response<LogChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_LOG)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_log(
            Utc.timestamp_nanos(request.timestamp * 1000),
            Uuid::from_slice(&request.device_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(LOG_DELETE_ERR))
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
