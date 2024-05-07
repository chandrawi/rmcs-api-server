use rmcs_resource_db::Resource;
use rmcs_resource_db::utility::migrate;
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
use rmcs_api_server::utility::interceptor::interceptor;
use rmcs_api_server::utility::validator::{AccessValidator, AccessSchema};
use rmcs_api_server::utility::config::{ROOT_DATA, RootData};
use rmcs_api_server::utility::auth::api_login;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL_RESOURCE").unwrap();
    let addr = std::env::var("BIND_ADDRESS_RESOURCE").unwrap().parse()?;
    let auth_addr = std::env::var("SERVER_ADDRESS_AUTH").unwrap();
    let scheme = auth_addr.split(":").next().unwrap();
    let auth_addr = 
        if vec!["http", "https"].contains(&scheme) { auth_addr } 
        else { String::from("http://") + auth_addr.as_str() };
    let api_id = std::env::var("API_ID").unwrap().parse()?;
    let password = std::env::var("API_PASSWORD").unwrap();

    let response = api_login(&auth_addr, api_id, &password).await
        .expect("Failed to get api definition from Auth server");
    let token_key = response.access_key;
    let root_key = response.root_key;
    let accesses: Vec<AccessSchema> = response.access_procedures
        .into_iter()
        .map(|s| s.into())
        .collect();

    let root_pw = std::env::var("ROOT_PASSWORD");
    let root_ad = std::env::var("ROOT_ACCESS_DURATION");
    let root_rd = std::env::var("ROOT_REFRESH_DURATION");
    if let (Ok(password), Ok(access_duration), Ok(refresh_duration)) = (root_pw, root_ad, root_rd) {
        ROOT_DATA.set(RootData::new_with_key(
            &password, 
            access_duration.parse()?, 
            refresh_duration.parse()?, 
            &root_key
        )).unwrap();
    }

    let resource_db = Resource::new_with_url(&url).await;
    migrate(&resource_db.pool).await.unwrap();

    let model_server = ModelServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let device_server = DeviceServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let group_server = GroupServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let data_server = DataServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let buffer_server = BufferServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let slice_server = SliceServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let log_server = LogServer::new(resource_db.clone()).with_validator(&token_key, &accesses);

    let model_server = tonic_web::enable(ModelServiceServer::with_interceptor(model_server, interceptor));
    let device_server = tonic_web::enable(DeviceServiceServer::with_interceptor(device_server, interceptor));
    let group_server = tonic_web::enable(GroupServiceServer::with_interceptor(group_server, interceptor));
    let data_server = tonic_web::enable(DataServiceServer::with_interceptor(data_server, interceptor));
    let buffer_server = tonic_web::enable(BufferServiceServer::with_interceptor(buffer_server, interceptor));
    let slice_server = tonic_web::enable(SliceServiceServer::with_interceptor(slice_server, interceptor));
    let log_server = tonic_web::enable(LogServiceServer::with_interceptor(log_server, interceptor));

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
        .accept_http1(true)
        .add_service(model_server)
        .add_service(device_server)
        .add_service(group_server)
        .add_service(data_server)
        .add_service(buffer_server)
        .add_service(slice_server)
        .add_service(log_server)
        .add_service(reflection_service?)
        .serve(addr)
        .await?;

    Ok(())
}
