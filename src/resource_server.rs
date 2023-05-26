use rmcs_resource_db::Resource;
use rmcs_resource_api::model::model_service_server::ModelServiceServer;
use rmcs_resource_api::device::device_service_server::DeviceServiceServer;
use rmcs_resource_api::group::group_service_server::GroupServiceServer;
use rmcs_resource_api::data::data_service_server::DataServiceServer;
use rmcs_resource_api::buffer::buffer_service_server::BufferServiceServer;
use rmcs_resource_api::slice::slice_service_server::SliceServiceServer;
use rmcs_resource_api::log::log_service_server::LogServiceServer;
use rmcs_resource_api::descriptor;
use rmcs_api_server::resource::model::ModelServer;
use rmcs_api_server::resource::device::DeviceServer;
use rmcs_api_server::resource::group::GroupServer;
use rmcs_api_server::resource::data::DataServer;
use rmcs_api_server::resource::buffer::BufferServer;
use rmcs_api_server::resource::slice::SliceServer;
use rmcs_api_server::resource::log::LogServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_RESOURCE_URL").unwrap();
    let addr = std::env::var("ADDRESS_RESOURCE").unwrap().parse()?;

    let resource_db = Resource::new_with_url(&url).await;
    let model_server = ModelServer::new(resource_db.clone());
    let device_server = DeviceServer::new(resource_db.clone());
    let group_server = GroupServer::new(resource_db.clone());
    let data_server = DataServer::new(resource_db.clone());
    let buffer_server = BufferServer::new(resource_db.clone());
    let slice_server = SliceServer::new(resource_db.clone());
    let log_server = LogServer::new(resource_db.clone());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(descriptor::model::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::device::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::group::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::data::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::buffer::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::slice::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::log::DESCRIPTOR_SET)
        .build();

    tonic::transport::Server::builder()
        .add_service(ModelServiceServer::new(model_server))
        .add_service(DeviceServiceServer::new(device_server))
        .add_service(GroupServiceServer::new(group_server))
        .add_service(DataServiceServer::new(data_server))
        .add_service(BufferServiceServer::new(buffer_server))
        .add_service(SliceServiceServer::new(slice_server))
        .add_service(LogServiceServer::new(log_server))
        .add_service(reflection_service?)
        .serve(addr)
        .await?;

    Ok(())
}
