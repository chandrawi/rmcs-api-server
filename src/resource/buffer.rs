use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_resource_db::{Resource, DataType, ArrayDataValue};
use rmcs_resource_api::buffer::buffer_service_server::BufferService;
use rmcs_resource_api::common::{self, ResponseStatus};
use rmcs_resource_api::buffer::{
    BufferSchema, BufferId, BufferSelector, BuffersSelector, BufferUpdate,
    BufferReadResponse, BufferListResponse, BufferCreateResponse, BufferChangeResponse,
    BufferStatus
};

pub struct BufferServer {
    pub resource_db: Resource
}

impl BufferServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db
        }
    }
}

#[tonic::async_trait]
impl BufferService for BufferServer {

    async fn read_buffer(&self, request: Request<BufferId>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_buffer(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Success.into())
        };
        Ok(Response::new(BufferReadResponse { result, status }))
    }

    async fn read_buffer_first(&self, request: Request<BufferSelector>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_first(
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Success.into())
        };
        Ok(Response::new(BufferReadResponse { result, status }))
    }

    async fn read_buffer_last(&self, request: Request<BufferSelector>)
        -> Result<Response<BufferReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_buffer_last(
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Success.into())
        };
        Ok(Response::new(BufferReadResponse { result, status }))
    }

    async fn list_buffer_first(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_first(
            request.number,
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(BufferListResponse { results, status }))
    }

    async fn list_buffer_last(&self, request: Request<BuffersSelector>)
        -> Result<Response<BufferListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_buffer_last(
            request.number,
            request.device_id,
            request.model_id,
            request.status.map(|s| BufferStatus::from_i32(s).unwrap_or_default().as_str_name())
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(BufferListResponse { results, status }))
    }

    async fn create_buffer(&self, request: Request<BufferSchema>)
        -> Result<Response<BufferCreateResponse>, Status>
    {
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
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(BufferCreateResponse { id, status }))
    }

    async fn update_buffer(&self, request: Request<BufferUpdate>)
        -> Result<Response<BufferChangeResponse>, Status>
    {
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
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(BufferChangeResponse { status }))
    }

    async fn delete_buffer(&self, request: Request<BufferId>)
        -> Result<Response<BufferChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_buffer(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(BufferChangeResponse { status }))
    }

}
