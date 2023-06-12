use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_resource_db::Resource;
use rmcs_resource_api::slice::slice_service_server::SliceService;
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

const SLICE_NOT_FOUND: &str = "requested slice not found";
const SLICE_CREATE_ERR: &str = "create slice error";
const SLICE_UPDATE_ERR: &str = "update slice error";
const SLICE_DELETE_ERR: &str = "delete slice error";

#[tonic::async_trait]
impl SliceService for SliceServer {

    async fn read_slice(&self, request: Request<SliceId>)
        -> Result<Response<SliceReadResponse>, Status>
    {
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
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_device(request.device_id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(SLICE_NOT_FOUND))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_model(&self, request: Request<SliceModel>)
        -> Result<Response<SliceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_model(request.model_id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(SLICE_NOT_FOUND))
        };
        Ok(Response::new(SliceListResponse { results }))
    }

    async fn list_slice_by_device_model(&self, request: Request<SliceDeviceModel>)
        -> Result<Response<SliceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_slice_by_device_model(
            request.device_id,
            request.model_id
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
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(SLICE_CREATE_ERR))
        };
        Ok(Response::new(SliceCreateResponse { id }))
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
        match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(SLICE_UPDATE_ERR))
        };
        Ok(Response::new(SliceChangeResponse { }))
    }

    async fn delete_slice(&self, request: Request<SliceId>)
    -> Result<Response<SliceChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_slice(request.id).await;
        match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(SLICE_DELETE_ERR))
        };
        Ok(Response::new(SliceChangeResponse { }))
    }

}
