use tonic::{Request, Response, Status};
use rmcs_resource_db::Resource;
use rmcs_resource_api::group::group_service_server::GroupService;
use rmcs_resource_api::common::ResponseStatus;
use rmcs_resource_api::group::{
    GroupModelSchema, GroupDeviceSchema, GroupId, GroupName, GroupCategory, GroupNameCategory, GroupUpdate,
    GroupModel, GroupDevice,
    GroupModelReadResponse, GroupModelListResponse, GroupCreateResponse, GroupChangeResponse,
    GroupDeviceReadResponse, GroupDeviceListResponse
};

pub struct GroupServer {
    pub resource_db: Resource
}

impl GroupServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db
        }
    }
}

#[tonic::async_trait]
impl GroupService for GroupServer {

    async fn read_group_model(&self, request: Request<GroupId>)
        -> Result<Response<GroupModelReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_group_model(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupModelReadResponse { result, status }))
    }

    async fn list_group_model_by_name(&self, request: Request<GroupName>)
        -> Result<Response<GroupModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_group_model_by_name(&request.name).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupModelListResponse { results, status }))
    }

    async fn list_group_model_by_category(&self, request: Request<GroupCategory>)
        -> Result<Response<GroupModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_group_model_by_category(&request.category).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupModelListResponse { results, status }))
    }

    async fn list_group_model_by_name_category(&self, request: Request<GroupNameCategory>)
        -> Result<Response<GroupModelListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_group_model_by_name_category(
            &request.name,
            &request.category
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupModelListResponse { results, status }))
    }

    async fn create_group_model(&self, request: Request<GroupModelSchema>)
        -> Result<Response<GroupCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.create_group_model(
            &request.name,
            &request.category,
            Some(&request.description)
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupCreateResponse { id, status }))
    }

    async fn update_group_model(&self, request: Request<GroupUpdate>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.update_group_model(
            request.id,
            request.name.as_deref(),
            request.category.as_deref(),
            request.description.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

    async fn delete_group_model(&self, request: Request<GroupId>)
    -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_group_model(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

    async fn add_group_model_member(&self, request: Request<GroupModel>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.add_group_model_member(
            request.id,
            request.model_id
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

    async fn remove_group_model_member(&self, request: Request<GroupModel>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.remove_group_model_member(
            request.id,
            request.model_id
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

    async fn read_group_device(&self, request: Request<GroupId>)
        -> Result<Response<GroupDeviceReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_group_device(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupDeviceReadResponse { result, status }))
    }

    async fn list_group_device_by_name(&self, request: Request<GroupName>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_group_device_by_name(&request.name).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupDeviceListResponse { results, status }))
    }

    async fn list_group_device_by_category(&self, request: Request<GroupCategory>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_group_device_by_category(&request.category).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupDeviceListResponse { results, status }))
    }

    async fn list_group_device_by_name_category(&self, request: Request<GroupNameCategory>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_group_device_by_name_category(
            &request.name,
            &request.category
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupDeviceListResponse { results, status }))
    }

    async fn create_group_device(&self, request: Request<GroupDeviceSchema>)
        -> Result<Response<GroupCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.create_group_device(
            &request.name,
            &request.category,
            Some(&request.description)
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(GroupCreateResponse { id, status }))
    }

    async fn update_group_device(&self, request: Request<GroupUpdate>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.update_group_device(
            request.id,
            request.name.as_deref(),
            request.category.as_deref(),
            request.description.as_deref()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

    async fn delete_group_device(&self, request: Request<GroupId>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_group_device(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

    async fn add_group_device_member(&self, request: Request<GroupDevice>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.add_group_device_member(
            request.id,
            request.device_id
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

    async fn remove_group_device_member(&self, request: Request<GroupDevice>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.remove_group_device_member(
            request.id,
            request.device_id
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(GroupChangeResponse { status }))
    }

}
