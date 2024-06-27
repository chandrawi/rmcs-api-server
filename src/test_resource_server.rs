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
use rmcs_api_server::utility::interceptor::interceptor;
use rmcs_api_server::utility::validator::{AccessValidator, AccessSchema};
use rmcs_api_server::utility::auth::api_login;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use http::{header::HeaderName, Method};
use tower_http::cors::{CorsLayer, Any};
use uuid::Uuid;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    db_url: Option<String>,
    #[arg(long)]
    address: Option<String>,
    #[arg(long)]
    secured: bool,
    #[arg(long)]
    auth_address: Option<String>,
    #[arg(long)]
    api_id: Option<String>,
    #[arg(long)]
    password: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    let db_url = match args.db_url {
        Some(value) => value,
        None => std::env::var("DATABASE_URL_RESOURCE_TEST").unwrap()
    };
    let address = match args.address {
        Some(value) => value,
        None => std::env::var("BIND_ADDRESS_RESOURCE").unwrap()
    };
    let mut auth_address = match args.auth_address {
        Some(value) => value,
        None => std::env::var("SERVER_ADDRESS_AUTH").unwrap()
    };
    let scheme = auth_address.split(":").next().unwrap();
    if !(vec!["http", "https"].contains(&scheme)) {
        auth_address = String::from("http://") + auth_address.as_str();
    }
    let api_id = match args.api_id {
        Some(value) => value,
        None => std::env::var("API_ID").unwrap()
    };
    let password = match args.password {
        Some(value) => value,
        None => std::env::var("API_PASSWORD").unwrap()
    };

    if args.secured {
        resource_server_secured(db_url, address, auth_address, api_id, password).await
    } else {
        resource_server(db_url, address).await
    }
}

async fn resource_server(db_url: String, address: String) -> Result<(), Box<dyn std::error::Error>>
{
    let addr = address.parse()?;

    let resource_db = Resource::new_with_url(&db_url).await;
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

    let model_server = ModelServiceServer::new(model_server);
    let device_server = DeviceServiceServer::new(device_server);
    let group_server = GroupServiceServer::new(group_server);
    let data_server = DataServiceServer::new(data_server);
    let buffer_server = BufferServiceServer::new(buffer_server);
    let slice_server = SliceServiceServer::new(slice_server);
    let log_server = LogServiceServer::new(log_server);

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
            .allow_methods([Method::POST])
            .expose_headers([HeaderName::from_static("grpc-status"), HeaderName::from_static("grpc-message")])
        )
        .layer(GrpcWebLayer::new())
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

async fn resource_server_secured(db_url: String, address: String, auth_address: String, api_id: String, password: String) -> Result<(), Box<dyn std::error::Error>> 
{
    let addr = address.parse()?;
    let api_id = Uuid::try_parse(&api_id).unwrap();

    let response = api_login(&auth_address, api_id, &password).await
        .expect("Failed to get api definition from Auth server");
    let token_key = response.access_key;
    let accesses: Vec<AccessSchema> = response.access_procedures
        .into_iter()
        .map(|s| s.into())
        .collect();

    let resource_db = Resource::new_with_url(&db_url).await;
    let model_server = ModelServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let device_server = DeviceServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let group_server = GroupServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let data_server = DataServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let buffer_server = BufferServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let slice_server = SliceServer::new(resource_db.clone()).with_validator(&token_key, &accesses);
    let log_server = LogServer::new(resource_db.clone()).with_validator(&token_key, &accesses);

    let model_server = ModelServiceServer::with_interceptor(model_server, interceptor);
    let device_server = DeviceServiceServer::with_interceptor(device_server, interceptor);
    let group_server = GroupServiceServer::with_interceptor(group_server, interceptor);
    let data_server = DataServiceServer::with_interceptor(data_server, interceptor);
    let buffer_server = BufferServiceServer::with_interceptor(buffer_server, interceptor);
    let slice_server = SliceServiceServer::with_interceptor(slice_server, interceptor);
    let log_server = LogServiceServer::with_interceptor(log_server, interceptor);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(descriptor::model::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::device::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::group::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::data::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::buffer::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::slice::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::log::DESCRIPTOR_SET)
        .build();

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
            .allow_methods([Method::POST])
            .expose_headers([HeaderName::from_static("grpc-status"), HeaderName::from_static("grpc-message")])
        )
        .layer(GrpcWebLayer::new())
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
