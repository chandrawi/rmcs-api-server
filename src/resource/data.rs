use tonic::{Request, Response, Status};
use chrono::{Utc, TimeZone};
use rmcs_resource_db::{Resource, DataType, ArrayDataValue};
use rmcs_resource_api::data::data_service_server::DataService;
use rmcs_resource_api::common::{self, ResponseStatus};
use rmcs_resource_api::data::{
    DataSchema, DataId, DataTime, DataRange, DataNumber,
    ModelId, DataIdModel, DataTimeModel, DataRangeModel, DataNumberModel,
    DataReadResponse, DataListResponse, DataModelResponse, DataChangeResponse, DataSchemaModel
};

pub struct DataServer {
    pub resource_db: Resource
}

impl DataServer {
    pub fn new(resource_db: Resource) -> Self {
        Self {
            resource_db
        }
    }
}

#[tonic::async_trait]
impl DataService for DataServer {

    async fn read_data(&self, request: Request<DataId>)
        -> Result<Response<DataReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.read_data(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp),
            Some(request.index as u16)
        ).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataReadResponse { result, status }))
    }

    async fn list_data_by_time(&self, request: Request<DataTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_time(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp)
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_by_last_time(&self, request: Request<DataTime>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_last_time(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp)
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_by_range_time(&self, request: Request<DataRange>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_range_time(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.begin),
            Utc.timestamp_nanos(request.end)
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_by_number_before(&self, request: Request<DataNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_number_before(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp),
            request.number
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_by_number_after(&self, request: Request<DataNumber>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.list_data_by_number_after(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp),
            request.number
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn get_data_model(&self, request: Request<ModelId>)
        -> Result<Response<DataModelResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.get_data_model(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataModelResponse { result, status }))
    }

    async fn read_data_with_model(&self, request: Request<DataIdModel>)
        -> Result<Response<DataReadResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataReadResponse { result: None, status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.read_data_with_model(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.timestamp),
            Some(request.index as u16)
        ).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataReadResponse { result, status }))
    }

    async fn list_data_with_model_by_time(&self, request: Request<DataTimeModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new(), status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.list_data_with_model_by_time(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.timestamp)
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_with_model_by_last_time(&self, request: Request<DataTimeModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new(), status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.list_data_with_model_by_last_time(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.timestamp)
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_with_model_by_range_time(&self, request: Request<DataRangeModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new(), status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.list_data_with_model_by_range_time(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.begin),
            Utc.timestamp_nanos(request.end)
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_with_model_by_number_before(&self, request: Request<DataNumberModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new(), status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.list_data_with_model_by_number_before(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.timestamp),
            request.number
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn list_data_with_model_by_number_after(&self, request: Request<DataNumberModel>)
        -> Result<Response<DataListResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataListResponse { results: Vec::new(), status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.list_data_with_model_by_number_after(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.timestamp),
            request.number
        ).await;
        let (results, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(DataListResponse { results, status }))
    }

    async fn create_data(&self, request: Request<DataSchema>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.create_data(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp),
            Some(request.index as u16),
            ArrayDataValue::from_bytes(
                &request.data_bytes,
                request.data_type.into_iter().map(|e| {
                    DataType::from(common::DataType::from_i32(e).unwrap_or_default())
                }).collect::<Vec<DataType>>().as_slice()
            ).to_vec()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(DataChangeResponse { status }))
    }

    async fn create_data_with_model(&self, request: Request<DataSchemaModel>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataChangeResponse { status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.create_data_with_model(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.timestamp),
            Some(request.index as u16),
            ArrayDataValue::from_bytes(
                &request.data_bytes,
                request.data_type.into_iter().map(|e| {
                    DataType::from(common::DataType::from_i32(e).unwrap_or_default())
                }).collect::<Vec<DataType>>().as_slice()
            ).to_vec()
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(DataChangeResponse { status }))
    }

    async fn delete_data(&self, request: Request<DataId>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.resource_db.delete_data(
            request.device_id,
            request.model_id,
            Utc.timestamp_nanos(request.timestamp),
            Some(request.index as u16)
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(DataChangeResponse { status }))
    }

    async fn delete_data_with_model(&self, request: Request<DataIdModel>)
        -> Result<Response<DataChangeResponse>, Status>
    {
        let request = request.into_inner();
        if let None = request.model {
            return Ok(Response::new(DataChangeResponse { status: ResponseStatus::Failed.into() }));
        }
        let result = self.resource_db.delete_data_with_model(
            request.model.unwrap().into(),
            request.device_id,
            Utc.timestamp_nanos(request.timestamp),
            Some(request.index as u16)
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(DataChangeResponse { status }))
    }

}
