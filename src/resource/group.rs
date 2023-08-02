use tonic::{Request, Response, Status};
use uuid::Uuid;
use rmcs_resource_db::Resource;
use rmcs_resource_api::group::group_service_server::GroupService;
use rmcs_resource_api::group::{
    GroupModelSchema, GroupDeviceSchema, GroupId, GroupName, GroupCategory, GroupNameCategory, GroupUpdate,
    GroupModel, GroupDevice,
    GroupModelReadResponse, GroupModelListResponse, GroupCreateResponse, GroupChangeResponse,
    GroupDeviceReadResponse, GroupDeviceListResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_GROUP_MODEL, LIST_GROUP_MODEL_BY_NAME, LIST_GROUP_MODEL_BY_CATEGORY, LIST_GROUP_MODEL_BY_NAME_CATEGORY,
    CREATE_GROUP_MODEL, UPDATE_GROUP_MODEL, DELETE_GROUP_MODEL, ADD_GROUP_MODEL_MEMBER, REMOVE_GROUP_MODEL_MEMBER,
    READ_GROUP_DEVICE, LIST_GROUP_DEVICE_BY_NAME, LIST_GROUP_DEVICE_BY_CATEGORY, LIST_GROUP_DEVICE_BY_NAME_CATEGORY,
    CREATE_GROUP_DEVICE, UPDATE_GROUP_DEVICE, DELETE_GROUP_DEVICE, ADD_GROUP_DEVICE_MEMBER, REMOVE_GROUP_DEVICE_MEMBER,
    READ_GROUP_GATEWAY, LIST_GROUP_GATEWAY_BY_NAME, LIST_GROUP_GATEWAY_BY_CATEGORY, LIST_GROUP_GATEWAY_BY_NAME_CATEGORY,
    CREATE_GROUP_GATEWAY, UPDATE_GROUP_GATEWAY, DELETE_GROUP_GATEWAY, ADD_GROUP_GATEWAY_MEMBER, REMOVE_GROUP_GATEWAY_MEMBER
};
use super::{
    GROUP_NOT_FOUND, GROUP_CREATE_ERR, GROUP_UPDATE_ERR, GROUP_DELETE_ERR, ADD_MEMBER_ERR, RMV_MEMBER_ERR
};

pub struct GroupServer {
    resource_db: Resource,
    token_key: Vec<u8>,
    accesses: Vec<AccessSchema>
}

impl GroupServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db,
            token_key: Vec::new(),
            accesses: Vec::new()
        }
    }
}

#[tonic::async_trait]
impl GroupService for GroupServer {

    async fn read_group_model(&self, request: Request<GroupId>)
        -> Result<Response<GroupModelReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_GROUP_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.read_group_model(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupModelReadResponse { result }))
    }

    async fn list_group_model_by_name(&self, request: Request<GroupName>)
        -> Result<Response<GroupModelListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_MODEL_BY_NAME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_model_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupModelListResponse { results }))
    }

    async fn list_group_model_by_category(&self, request: Request<GroupCategory>)
        -> Result<Response<GroupModelListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_MODEL_BY_CATEGORY)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_model_by_category(&request.category).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupModelListResponse { results }))
    }

    async fn list_group_model_by_name_category(&self, request: Request<GroupNameCategory>)
        -> Result<Response<GroupModelListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_MODEL_BY_NAME_CATEGORY)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_model_by_name_category(
            &request.name,
            &request.category
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupModelListResponse { results }))
    }

    async fn create_group_model(&self, request: Request<GroupModelSchema>)
        -> Result<Response<GroupCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_GROUP_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.create_group_model(
            &request.name,
            &request.category,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(GROUP_CREATE_ERR))
        };
        Ok(Response::new(GroupCreateResponse { id: id.as_bytes().to_vec() }))
    }

    async fn update_group_model(&self, request: Request<GroupUpdate>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_GROUP_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.update_group_model(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            request.name.as_deref(),
            request.category.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GROUP_UPDATE_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn delete_group_model(&self, request: Request<GroupId>)
    -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_GROUP_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_group_model(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GROUP_DELETE_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn add_group_model_member(&self, request: Request<GroupModel>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), ADD_GROUP_MODEL_MEMBER)?;
        let request = request.into_inner();
        let result = self.resource_db.add_group_model_member(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_MEMBER_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn remove_group_model_member(&self, request: Request<GroupModel>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), REMOVE_GROUP_MODEL_MEMBER)?;
        let request = request.into_inner();
        let result = self.resource_db.remove_group_model_member(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            Uuid::from_slice(&request.model_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_MEMBER_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn read_group_device(&self, request: Request<GroupId>)
        -> Result<Response<GroupDeviceReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_GROUP_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.read_group_device(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceReadResponse { result }))
    }

    async fn list_group_device_by_name(&self, request: Request<GroupName>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_DEVICE_BY_NAME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_device_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceListResponse { results }))
    }

    async fn list_group_device_by_category(&self, request: Request<GroupCategory>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_DEVICE_BY_CATEGORY)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_device_by_category(&request.category).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceListResponse { results }))
    }

    async fn list_group_device_by_name_category(&self, request: Request<GroupNameCategory>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_DEVICE_BY_NAME_CATEGORY)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_device_by_name_category(
            &request.name,
            &request.category
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceListResponse { results }))
    }

    async fn create_group_device(&self, request: Request<GroupDeviceSchema>)
        -> Result<Response<GroupCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_GROUP_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.create_group_device(
            &request.name,
            &request.category,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(GROUP_CREATE_ERR))
        };
        Ok(Response::new(GroupCreateResponse { id: id.as_bytes().to_vec() }))
    }

    async fn update_group_device(&self, request: Request<GroupUpdate>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_GROUP_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.update_group_device(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            request.name.as_deref(),
            request.category.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GROUP_UPDATE_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn delete_group_device(&self, request: Request<GroupId>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_GROUP_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_group_device(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GROUP_DELETE_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn add_group_device_member(&self, request: Request<GroupDevice>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), ADD_GROUP_DEVICE_MEMBER)?;
        let request = request.into_inner();
        let result = self.resource_db.add_group_device_member(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            Uuid::from_slice(&request.device_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_MEMBER_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn remove_group_device_member(&self, request: Request<GroupDevice>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), REMOVE_GROUP_DEVICE_MEMBER)?;
        let request = request.into_inner();
        let result = self.resource_db.remove_group_device_member(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            Uuid::from_slice(&request.device_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_MEMBER_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn read_group_gateway(&self, request: Request<GroupId>)
        -> Result<Response<GroupDeviceReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_GROUP_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.read_group_gateway(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceReadResponse { result }))
    }

    async fn list_group_gateway_by_name(&self, request: Request<GroupName>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_GATEWAY_BY_NAME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_gateway_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceListResponse { results }))
    }

    async fn list_group_gateway_by_category(&self, request: Request<GroupCategory>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_GATEWAY_BY_CATEGORY)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_gateway_by_category(&request.category).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceListResponse { results }))
    }

    async fn list_group_gateway_by_name_category(&self, request: Request<GroupNameCategory>)
        -> Result<Response<GroupDeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GROUP_GATEWAY_BY_NAME_CATEGORY)?;
        let request = request.into_inner();
        let result = self.resource_db.list_group_gateway_by_name_category(
            &request.name,
            &request.category
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GROUP_NOT_FOUND))
        };
        Ok(Response::new(GroupDeviceListResponse { results }))
    }

    async fn create_group_gateway(&self, request: Request<GroupDeviceSchema>)
        -> Result<Response<GroupCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_GROUP_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.create_group_gateway(
            &request.name,
            &request.category,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(GROUP_CREATE_ERR))
        };
        Ok(Response::new(GroupCreateResponse { id: id.as_bytes().to_vec() }))
    }

    async fn update_group_gateway(&self, request: Request<GroupUpdate>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_GROUP_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.update_group_gateway(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            request.name.as_deref(),
            request.category.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GROUP_UPDATE_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn delete_group_gateway(&self, request: Request<GroupId>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_GROUP_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_group_gateway(Uuid::from_slice(&request.id).unwrap_or_default()).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GROUP_DELETE_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn add_group_gateway_member(&self, request: Request<GroupDevice>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), ADD_GROUP_GATEWAY_MEMBER)?;
        let request = request.into_inner();
        let result = self.resource_db.add_group_gateway_member(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            Uuid::from_slice(&request.device_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_MEMBER_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

    async fn remove_group_gateway_member(&self, request: Request<GroupDevice>)
        -> Result<Response<GroupChangeResponse>, Status>
    {
        self.validate(request.extensions(), REMOVE_GROUP_GATEWAY_MEMBER)?;
        let request = request.into_inner();
        let result = self.resource_db.remove_group_gateway_member(
            Uuid::from_slice(&request.id).unwrap_or_default(),
            Uuid::from_slice(&request.device_id).unwrap_or_default()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_MEMBER_ERR))
        };
        Ok(Response::new(GroupChangeResponse { }))
    }

}

impl AccessValidator for GroupServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_GROUP_MODEL, LIST_GROUP_MODEL_BY_NAME, LIST_GROUP_MODEL_BY_CATEGORY, LIST_GROUP_MODEL_BY_NAME_CATEGORY,
            CREATE_GROUP_MODEL, UPDATE_GROUP_MODEL, DELETE_GROUP_MODEL, ADD_GROUP_MODEL_MEMBER, REMOVE_GROUP_MODEL_MEMBER,
            READ_GROUP_DEVICE, LIST_GROUP_DEVICE_BY_NAME, LIST_GROUP_DEVICE_BY_CATEGORY, LIST_GROUP_DEVICE_BY_NAME_CATEGORY,
            CREATE_GROUP_DEVICE, UPDATE_GROUP_DEVICE, DELETE_GROUP_DEVICE, ADD_GROUP_DEVICE_MEMBER, REMOVE_GROUP_DEVICE_MEMBER,
            READ_GROUP_GATEWAY, LIST_GROUP_GATEWAY_BY_NAME, LIST_GROUP_GATEWAY_BY_CATEGORY, LIST_GROUP_GATEWAY_BY_NAME_CATEGORY,
            CREATE_GROUP_GATEWAY, UPDATE_GROUP_GATEWAY, DELETE_GROUP_GATEWAY, ADD_GROUP_GATEWAY_MEMBER, REMOVE_GROUP_GATEWAY_MEMBER
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
