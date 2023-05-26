use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_resource_db::{Resource, LogType, LogValue};
use rmcs_resource_api::log::log_service_server::LogService;
use rmcs_resource_api::common::{self, ResponseStatus};
use rmcs_resource_api::log::{
    LogSchema, LogId, LogTime, LogRange, LogUpdate,
    LogReadResponse, LogListResponse, LogChangeResponse,
    LogStatus
};

pub struct LogServer {
    pub resource_db: Resource
}

impl LogServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db
        }
    }
}

#[tonic::async_trait]
impl LogService for LogServer {

    async fn read_log(&self, request: Request<LogId>)
        -> Result<Response<LogReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_log(
            Utc.timestamp_nanos(request.timestamp),
            request.device_id
        ).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Success.into())
        };
        Ok(Response::new(LogReadResponse { result, status }))
    }

    async fn list_log_by_time(&self, request: Request<LogTime>)
        -> Result<Response<LogListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_time(
            Utc.timestamp_nanos(request.timestamp),
            request.device_id,
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(LogListResponse { results, status }))
    }

    async fn list_log_by_last_time(&self, request: Request<LogTime>)
        -> Result<Response<LogListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_last_time(
            Utc.timestamp_nanos(request.timestamp),
            request.device_id,
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(LogListResponse { results, status }))
    }

    async fn list_log_by_range_time(&self, request: Request<LogRange>)
        -> Result<Response<LogListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_log_by_range_time(
            Utc.timestamp_nanos(request.begin),
            Utc.timestamp_nanos(request.end),
            request.device_id,
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(LogListResponse { results, status }))
    }

    async fn create_log(&self, request: Request<LogSchema>)
        -> Result<Response<LogChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.create_log(
            Utc.timestamp_nanos(request.timestamp),
            request.device_id,
            LogStatus::from_i32(request.status).unwrap_or_default().as_str_name(),
            LogValue::from_bytes(
                &request.log_bytes, 
                LogType::from(common::ConfigType::from_i32(request.log_type).unwrap_or_default())
            )
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(LogChangeResponse { status }))
    }

    async fn update_log(&self, request: Request<LogUpdate>)
    -> Result<Response<LogChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.update_log(
            Utc.timestamp_nanos(request.timestamp),
            request.device_id,
            request.status.map(|s| LogStatus::from_i32(s).unwrap_or_default().as_str_name()),
            request.log_bytes.map(|s| {
                LogValue::from_bytes(
                    &s, 
                    LogType::from(common::ConfigType::from_i32(request.log_type.unwrap_or_default()).unwrap_or_default())
                )
            })
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(LogChangeResponse { status }))
    }

    async fn delete_log(&self, request: Request<LogId>)
        -> Result<Response<LogChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_log(
            Utc.timestamp_nanos(request.timestamp),
            request.device_id
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(LogChangeResponse { status }))
    }

}
