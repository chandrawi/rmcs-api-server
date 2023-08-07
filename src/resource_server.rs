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
use rmcs_api_server::utility::interceptor;
use rmcs_api_server::utility::validator::{AccessValidator, AccessSchema};
use rmcs_api_server::utility::config::{ROOT_DATA, RootData};
use rmcs_api_client::Auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_RESOURCE_URL").unwrap();
    let addr = std::env::var("ADDRESS_RESOURCE").unwrap().parse()?;
    let auth_addr = String::from("http://") + std::env::var("ADDRESS_AUTH").unwrap().as_str();
    let api_id = std::env::var("API_ID").unwrap().parse()?;
    let password = std::env::var("API_PASSWORD").unwrap();

    let root_pw = std::env::var("ROOT_PASSWORD");
    let root_ad = std::env::var("ROOT_ACCESS_DURATION");
    let root_rd = std::env::var("ROOT_REFRESH_DURATION");
    if let (Ok(password), Ok(access_duration), Ok(refresh_duration)) = (root_pw, root_ad, root_rd) {
        ROOT_DATA.set(RootData::new(&password, access_duration.parse()?, refresh_duration.parse()?)).unwrap();
    }

    let auth = Auth::new(&auth_addr).await;
    let response = auth.api_login(api_id, &password).await
        .expect("Failed to get api definition from Auth server");
    let token_key = response.access_key;
    let accesses: Vec<AccessSchema> = response.access_procedures
        .into_iter()
        .map(|s| s.into())
        .collect();

    let resource_db = Resource::new_with_url(&url).await;
    let model_server = ModelServer::new(resource_db.clone())
        .with_validator(&token_key, &accesses);
    let device_server = DeviceServer::new(resource_db.clone())
        .with_validator(&token_key, &accesses);
    let group_server = GroupServer::new(resource_db.clone())
        .with_validator(&token_key, &accesses);
    let data_server = DataServer::new(resource_db.clone())
        .with_validator(&token_key, &accesses);
    let buffer_server = BufferServer::new(resource_db.clone())
        .with_validator(&token_key, &accesses);
    let slice_server = SliceServer::new(resource_db.clone())
        .with_validator(&token_key, &accesses);
    let log_server = LogServer::new(resource_db.clone())
        .with_validator(&token_key, &accesses);

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
        .add_service(ModelServiceServer::with_interceptor(model_server, interceptor))
        .add_service(DeviceServiceServer::with_interceptor(device_server, interceptor))
        .add_service(GroupServiceServer::with_interceptor(group_server, interceptor))
        .add_service(DataServiceServer::with_interceptor(data_server, interceptor))
        .add_service(BufferServiceServer::with_interceptor(buffer_server, interceptor))
        .add_service(SliceServiceServer::with_interceptor(slice_server, interceptor))
        .add_service(LogServiceServer::with_interceptor(log_server, interceptor))
        .add_service(reflection_service?)
        .serve(addr)
        .await?;

    Ok(())
}
