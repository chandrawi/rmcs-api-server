use tonic::{Request, Response, Status};
use rmcs_resource_db::{Resource, ConfigType, ConfigValue};
use rmcs_resource_api::device::device_service_server::DeviceService;
use rmcs_resource_api::common;
use rmcs_resource_api::device::{
    DeviceSchema, DeviceId, SerialNumber, DeviceName, DeviceGatewayName, DeviceGatewayType, DeviceUpdate,
    GatewaySchema, GatewayId, GatewayName, GatewayUpdate,
    ConfigSchema, ConfigId, ConfigUpdate,
    TypeSchema, TypeId, TypeName, TypeModel, TypeUpdate,
    DeviceReadResponse, DeviceListResponse, DeviceChangeResponse,
    GatewayReadResponse, GatewayListResponse, GatewayChangeResponse,
    ConfigReadResponse, ConfigListResponse, ConfigCreateResponse, ConfigChangeResponse,
    TypeReadResponse, TypeListResponse, TypeCreateResponse, TypeChangeResponse
};
use crate::utility::validator::{AccessValidator, AccessSchema};
use super::{
    READ_DEVICE, READ_DEVICE_BY_SN, LIST_DEVICE_BY_GATEWAY, LIST_DEVICE_BY_TYPE, LIST_DEVICE_BY_NAME,
    LIST_DEVICE_BY_GATEWAY_TYPE, LIST_DEVICE_BY_GATEWAY_NAME,
    CREATE_DEVICE, UPDATE_DEVICE, DELETE_DEVICE,
    READ_GATEWAY, READ_GATEWAY_BY_SN, LIST_GATEWAY_BY_TYPE, LIST_GATEWAY_BY_NAME,
    CREATE_GATEWAY, UPDATE_GATEWAY, DELETE_GATEWAY,
    READ_DEVICE_CONFIG, LIST_DEVICE_CONFIG, CREATE_DEVICE_CONFIG, UPDATE_DEVICE_CONFIG, DELETE_DEVICE_CONFIG,
    READ_GATEWAY_CONFIG, LIST_GATEWAY_CONFIG, CREATE_GATEWAY_CONFIG, UPDATE_GATEWAY_CONFIG, DELETE_GATEWAY_CONFIG,
    READ_TYPE, LIST_TYPE_BY_NAME, CREATE_TYPE, UPDATE_TYPE, DELETE_TYPE, ADD_TYPE_MODEL, REMOVE_TYPE_MODEL
};
use super::{
    DEVICE_NOT_FOUND, DEVICE_CREATE_ERR, DEVICE_UPDATE_ERR, DEVICE_DELETE_ERR,
    GATEWAY_NOT_FOUND, GATEWAY_CREATE_ERR, GATEWAY_UPDATE_ERR, GATEWAY_DELETE_ERR,
    CFG_NOT_FOUND, CFG_CREATE_ERR, CFG_UPDATE_ERR, CFG_DELETE_ERR,
    TYPE_NOT_FOUND, TYPE_CREATE_ERR, TYPE_UPDATE_ERR, TYPE_DELETE_ERR,
    ADD_TYPE_ERR, RMV_TYPE_ERR
};

#[derive(Debug)]
pub struct DeviceServer {
    resource_db: Resource,
    token_key: Vec<u8>,
    accesses: Vec<AccessSchema>
}

impl DeviceServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db,
            token_key: Vec::new(),
            accesses: Vec::new()
        }
    }
}

#[tonic::async_trait]
impl DeviceService for DeviceServer {
    async fn read_device(&self, request: Request<DeviceId>)
        -> Result<Response<DeviceReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.read_device(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(DEVICE_NOT_FOUND))
        };
        Ok(Response::new(DeviceReadResponse { result }))
    }

    async fn read_device_by_sn(&self, request: Request<SerialNumber>)
        -> Result<Response<DeviceReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DEVICE_BY_SN)?;
        let request = request.into_inner();
        let result = self.resource_db.read_device_by_sn(&request.serial_number).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(DEVICE_NOT_FOUND))
        };
        Ok(Response::new(DeviceReadResponse { result }))
    }

    async fn list_device_by_gateway(&self, request: Request<GatewayId>)
        -> Result<Response<DeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DEVICE_BY_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.list_device_by_gateway(request.id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DEVICE_NOT_FOUND))
        };
        Ok(Response::new(DeviceListResponse { results }))
    }

    async fn list_device_by_type(&self, request: Request<TypeId>)
        -> Result<Response<DeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DEVICE_BY_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_device_by_type(request.id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DEVICE_NOT_FOUND))
        };
        Ok(Response::new(DeviceListResponse { results }))
    }

    async fn list_device_by_name(&self, request: Request<DeviceName>)
        -> Result<Response<DeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DEVICE_BY_NAME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_device_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DEVICE_NOT_FOUND))
        };
        Ok(Response::new(DeviceListResponse { results }))
    }

    async fn list_device_by_gateway_type(&self, request: Request<DeviceGatewayType>)
        -> Result<Response<DeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DEVICE_BY_GATEWAY_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_device_by_gateway_type(
            request.gateway_id,
            request.type_id
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DEVICE_NOT_FOUND))
        };
        Ok(Response::new(DeviceListResponse { results }))
    }

    async fn list_device_by_gateway_name(&self, request: Request<DeviceGatewayName>)
        -> Result<Response<DeviceListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DEVICE_BY_GATEWAY_NAME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_device_by_gateway_name(
            request.gateway_id,
            &request.name
        ).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(DEVICE_NOT_FOUND))
        };
        Ok(Response::new(DeviceListResponse { results }))
    }

    async fn create_device(&self, request: Request<DeviceSchema>)
        -> Result<Response<DeviceChangeResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.create_device(
            request.id,
            request.gateway_id,
            request.device_type.unwrap_or_default().id,
            &request.serial_number,
            &request.name,
            Some(&request.description)
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DEVICE_CREATE_ERR))
        };
        Ok(Response::new(DeviceChangeResponse { }))
    }

    async fn update_device(&self, request: Request<DeviceUpdate>)
        -> Result<Response<DeviceChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.update_device(
            request.id,
            request.gateway_id,
            request.type_id,
            request.serial_number.as_deref(),
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DEVICE_UPDATE_ERR))
        };
        Ok(Response::new(DeviceChangeResponse { }))
    }

    async fn delete_device(&self, request: Request<DeviceId>)
        -> Result<Response<DeviceChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_DEVICE)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_device(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(DEVICE_DELETE_ERR))
        };
        Ok(Response::new(DeviceChangeResponse { }))
    }

    async fn read_gateway(&self, request: Request<GatewayId>)
        -> Result<Response<GatewayReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.read_gateway(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(GATEWAY_NOT_FOUND))
        };
        Ok(Response::new(GatewayReadResponse { result }))
    }

    async fn read_gateway_by_sn(&self, request: Request<SerialNumber>)
        -> Result<Response<GatewayReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_GATEWAY_BY_SN)?;
        let request = request.into_inner();
        let result = self.resource_db.read_gateway_by_sn(&request.serial_number).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(GATEWAY_NOT_FOUND))
        };
        Ok(Response::new(GatewayReadResponse { result }))
    }

    async fn list_gateway_by_type(&self, request: Request<TypeId>)
        -> Result<Response<GatewayListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GATEWAY_BY_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.list_gateway_by_type(request.id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GATEWAY_NOT_FOUND))
        };
        Ok(Response::new(GatewayListResponse { results }))
    }

    async fn list_gateway_by_name(&self, request: Request<GatewayName>)
        -> Result<Response<GatewayListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GATEWAY_BY_NAME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_gateway_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(GATEWAY_NOT_FOUND))
        };
        Ok(Response::new(GatewayListResponse { results }))
    }

    async fn create_gateway(&self, request: Request<GatewaySchema>)
        -> Result<Response<GatewayChangeResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.create_gateway(
            request.id,
            request.gateway_type.unwrap_or_default().id,
            &request.serial_number,
            &request.name,
            Some(&request.description)
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GATEWAY_CREATE_ERR))
        };
        Ok(Response::new(GatewayChangeResponse { }))
    }

    async fn update_gateway(&self, request: Request<GatewayUpdate>)
        -> Result<Response<GatewayChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.update_gateway(
            request.id,
            request.type_id,
            request.serial_number.as_deref(),
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GATEWAY_UPDATE_ERR))
        };
        Ok(Response::new(GatewayChangeResponse { }))
    }

    async fn delete_gateway(&self, request: Request<GatewayId>)
    -> Result<Response<GatewayChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_GATEWAY)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_gateway(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(GATEWAY_DELETE_ERR))
        };
        Ok(Response::new(GatewayChangeResponse { }))
    }

    async fn read_device_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_DEVICE_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.read_device_config(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(CFG_NOT_FOUND))
        };
        Ok(Response::new(ConfigReadResponse { result }))
    }

    async fn list_device_config(&self, request: Request<DeviceId>)
        -> Result<Response<ConfigListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_DEVICE_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_device_config_by_device(request.id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(CFG_NOT_FOUND))
        };
        Ok(Response::new(ConfigListResponse { results }))
    }

    async fn create_device_config(&self, request: Request<ConfigSchema>)
        -> Result<Response<ConfigCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_DEVICE_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.create_device_config(
            request.device_id,
            &request.name,
            ConfigValue::from_bytes(
                &request.config_bytes, 
                ConfigType::from(common::ConfigType::from_i32(request.config_type).unwrap_or_default())
            ),
            &request.category
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(CFG_CREATE_ERR))
        };
        Ok(Response::new(ConfigCreateResponse { id }))
    }

    async fn update_device_config(&self, request: Request<ConfigUpdate>)
        -> Result<Response<ConfigChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_DEVICE_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.update_device_config(
            request.id,
            request.name.as_deref(),
            request.config_bytes.map(|s| {
                ConfigValue::from_bytes(
                    &s,
                    ConfigType::from(common::ConfigType::from_i32(request.config_type.unwrap_or_default()).unwrap_or_default())
                )
            }),
            request.category.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(CFG_UPDATE_ERR))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

    async fn delete_device_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_DEVICE_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_device_config(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(CFG_DELETE_ERR))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

    async fn read_gateway_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_GATEWAY_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.read_gateway_config(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(CFG_NOT_FOUND))
        };
        Ok(Response::new(ConfigReadResponse { result }))
    }

    async fn list_gateway_config(&self, request: Request<GatewayId>)
        -> Result<Response<ConfigListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_GATEWAY_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.list_gateway_config_by_gateway(request.id).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(CFG_NOT_FOUND))
        };
        Ok(Response::new(ConfigListResponse { results }))
    }

    async fn create_gateway_config(&self, request: Request<ConfigSchema>)
        -> Result<Response<ConfigCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_GATEWAY_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.create_gateway_config(
            request.device_id,
            &request.name,
            ConfigValue::from_bytes(
                &request.config_bytes, 
                ConfigType::from(common::ConfigType::from_i32(request.config_type).unwrap_or_default())
            ),
            &request.category
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(CFG_CREATE_ERR))
        };
        Ok(Response::new(ConfigCreateResponse { id }))
    }

    async fn update_gateway_config(&self, request: Request<ConfigUpdate>)
        -> Result<Response<ConfigChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_GATEWAY_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.update_gateway_config(
            request.id,
            request.name.as_deref(),
            request.config_bytes.map(|s| {
                ConfigValue::from_bytes(
                    &s,
                    ConfigType::from(common::ConfigType::from_i32(request.config_type.unwrap_or_default()).unwrap_or_default())
                )
            }),
            request.category.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(CFG_UPDATE_ERR))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

    async fn delete_gateway_config(&self, request: Request<ConfigId>)
        -> Result<Response<ConfigChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_GATEWAY_CONFIG)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_gateway_config(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(CFG_DELETE_ERR))
        };
        Ok(Response::new(ConfigChangeResponse { }))
    }

    async fn read_type(&self, request: Request<TypeId>)
        -> Result<Response<TypeReadResponse>, Status>
    {
        self.validate(request.extensions(), READ_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.read_type(request.id).await;
        let result = match result {
            Ok(value) => Some(value.into()),
            Err(_) => return Err(Status::not_found(TYPE_NOT_FOUND))
        };
        Ok(Response::new(TypeReadResponse { result }))
    }

    async fn list_type_by_name(&self, request: Request<TypeName>)
        -> Result<Response<TypeListResponse>, Status>
    {
        self.validate(request.extensions(), LIST_TYPE_BY_NAME)?;
        let request = request.into_inner();
        let result = self.resource_db.list_type_by_name(&request.name).await;
        let results = match result {
            Ok(value) => value.into_iter().map(|e| e.into()).collect(),
            Err(_) => return Err(Status::not_found(TYPE_NOT_FOUND))
        };
        Ok(Response::new(TypeListResponse { results }))
    }

    async fn create_type(&self, request: Request<TypeSchema>)
        -> Result<Response<TypeCreateResponse>, Status>
    {
        self.validate(request.extensions(), CREATE_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.create_type(
            &request.name,
            Some(&request.description)
        ).await;
        let id = match result {
            Ok(value) => value,
            Err(_) => return Err(Status::internal(TYPE_CREATE_ERR))
        };
        Ok(Response::new(TypeCreateResponse { id }))
    }

    async fn update_type(&self, request: Request<TypeUpdate>)
        -> Result<Response<TypeChangeResponse>, Status>
    {
        self.validate(request.extensions(), UPDATE_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.update_type(
            request.id,
            request.name.as_deref(),
            request.description.as_deref()
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(TYPE_UPDATE_ERR))
        };
        Ok(Response::new(TypeChangeResponse { }))
    }

    async fn delete_type(&self, request: Request<TypeId>)
        -> Result<Response<TypeChangeResponse>, Status>
    {
        self.validate(request.extensions(), DELETE_TYPE)?;
        let request = request.into_inner();
        let result = self.resource_db.delete_type(request.id).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(TYPE_DELETE_ERR))
        };
        Ok(Response::new(TypeChangeResponse { }))
    }

    async fn add_type_model(&self, request: Request<TypeModel>)
        -> Result<Response<TypeChangeResponse>, Status>
    {
        self.validate(request.extensions(), ADD_TYPE_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.add_type_model(
            request.id,
            request.model_id
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(ADD_TYPE_ERR))
        };
        Ok(Response::new(TypeChangeResponse { }))
    }

    async fn remove_type_model(&self, request: Request<TypeModel>)
        -> Result<Response<TypeChangeResponse>, Status>
    {
        self.validate(request.extensions(), REMOVE_TYPE_MODEL)?;
        let request = request.into_inner();
        let result = self.resource_db.remove_type_model(
            request.id,
            request.model_id
        ).await;
        match result {
            Ok(_) => (),
            Err(_) => return Err(Status::internal(RMV_TYPE_ERR))
        };
        Ok(Response::new(TypeChangeResponse { }))
    }
}

impl AccessValidator for DeviceServer {

    fn with_validator(mut self, token_key: &[u8], accesses: &[AccessSchema]) -> Self {
        const PROCEDURES: &[&str] = &[
            READ_DEVICE, READ_DEVICE_BY_SN, LIST_DEVICE_BY_GATEWAY, LIST_DEVICE_BY_TYPE, LIST_DEVICE_BY_NAME,
            LIST_DEVICE_BY_GATEWAY_TYPE, LIST_DEVICE_BY_GATEWAY_NAME,
            CREATE_DEVICE, UPDATE_DEVICE, DELETE_DEVICE,
            READ_GATEWAY, READ_GATEWAY_BY_SN, LIST_GATEWAY_BY_TYPE, LIST_GATEWAY_BY_NAME,
            CREATE_GATEWAY, UPDATE_GATEWAY, DELETE_GATEWAY,
            READ_DEVICE_CONFIG, LIST_DEVICE_CONFIG, CREATE_DEVICE_CONFIG, UPDATE_DEVICE_CONFIG, DELETE_DEVICE_CONFIG,
            READ_GATEWAY_CONFIG, LIST_GATEWAY_CONFIG, CREATE_GATEWAY_CONFIG, UPDATE_GATEWAY_CONFIG, DELETE_GATEWAY_CONFIG,
            READ_TYPE, LIST_TYPE_BY_NAME, CREATE_TYPE, UPDATE_TYPE, DELETE_TYPE, ADD_TYPE_MODEL, REMOVE_TYPE_MODEL
        ];
        self.token_key = token_key.to_owned();
        self.accesses = PROCEDURES.into_iter().map(|&s| AccessSchema {
            procedure: s.to_owned(),
            roles: accesses.iter()
                .filter(|&a| a.procedure == s)
                .map(|a| a.roles.clone())
                .next()
                .unwrap_or_default()
        })
        .collect();
        self
    }

    fn token_key(&self) -> Vec<u8> {
        self.token_key.clone()
    }

    fn accesses(&self) -> Vec<AccessSchema> {
        self.accesses.clone()
    }

}
