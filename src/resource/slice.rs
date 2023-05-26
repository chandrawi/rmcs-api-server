use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_resource_db::Resource;
use rmcs_resource_api::slice::slice_service_server::SliceService;
use rmcs_resource_api::common::ResponseStatus;
use rmcs_resource_api::slice::{
    SliceSchema, SliceId, SliceName, SliceDevice, SliceModel, SliceDeviceModel, SliceUpdate,
    SliceReadResponse, SliceListResponse, SliceCreateResponse, SliceChangeResponse
};

pub struct SliceServer {
    pub resource_db: Resource
}

impl SliceServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db
        }
    }
}

#[tonic::async_trait]
impl SliceService for SliceServer {

    async fn read_slice(&self, request: Request<SliceId>)
        -> Result<Response<SliceReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_slice(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Success.into())
        };
        Ok(Response::new(SliceReadResponse { result, status }))
    }

    async fn list_slice_by_name(&self, request: Request<SliceName>)
        -> Result<Response<SliceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_name(&request.name).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(SliceListResponse { results, status }))
    }

    async fn list_slice_by_device(&self, request: Request<SliceDevice>)
        -> Result<Response<SliceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_device(request.device_id).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(SliceListResponse { results, status }))
    }

    async fn list_slice_by_model(&self, request: Request<SliceModel>)
        -> Result<Response<SliceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_model(request.model_id).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(SliceListResponse { results, status }))
    }

    async fn list_slice_by_device_model(&self, request: Request<SliceDeviceModel>)
        -> Result<Response<SliceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_device_model(
            request.device_id,
            request.model_id
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Success.into())
        };
        Ok(Response::new(SliceListResponse { results, status }))
    }

    async fn create_slice(&self, request: Request<SliceSchema>)
        -> Result<Response<SliceCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.create_slice(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp_begin),
            Utc.timestamp_nanos(request.timestamp_end),
            Some(request.index_begin as u16),
            Some(request.index_end as u16),
            &request.name,
            Some(&request.description)
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(SliceCreateResponse { id, status }))
    }

    async fn update_slice(&self, request: Request<SliceUpdate>)
        -> Result<Response<SliceChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.update_slice(
            request.id,
            request.timestamp_begin.map(|s| Utc.timestamp_nanos(s)),
            request.timestamp_end.map(|s| Utc.timestamp_nanos(s)),
            request.index_begin.map(|s| s as u16),
            request.index_end.map(|s| s as u16),
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(SliceChangeResponse { status }))
    }

    async fn delete_slice(&self, request: Request<SliceId>)
    -> Result<Response<SliceChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_slice(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(SliceChangeResponse { status }))
    }

}
