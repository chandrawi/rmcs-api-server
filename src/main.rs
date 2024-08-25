use rmcs_auth_db::Auth;
use rmcs_auth_api::api::api_service_server::ApiServiceServer;
use rmcs_auth_api::role::role_service_server::RoleServiceServer;
use rmcs_auth_api::user::user_service_server::UserServiceServer;
use rmcs_auth_api::profile::profile_service_server::ProfileServiceServer;
use rmcs_auth_api::token::token_service_server::TokenServiceServer;
use rmcs_auth_api::auth::auth_service_server::AuthServiceServer;
use rmcs_auth_api::descriptor as auth_descriptor;
use rmcs_resource_db::Resource;
use rmcs_resource_api::model::model_service_server::ModelServiceServer;
use rmcs_resource_api::device::device_service_server::DeviceServiceServer;
use rmcs_resource_api::group::group_service_server::GroupServiceServer;
use rmcs_resource_api::set::set_service_server::SetServiceServer;
use rmcs_resource_api::data::data_service_server::DataServiceServer;
use rmcs_resource_api::buffer::buffer_service_server::BufferServiceServer;
use rmcs_resource_api::slice::slice_service_server::SliceServiceServer;
use rmcs_resource_api::log::log_service_server::LogServiceServer;
use rmcs_resource_api::descriptor as resource_descriptor;
use rmcs_api_server::auth::api::ApiServer;
use rmcs_api_server::auth::role::RoleServer;
use rmcs_api_server::auth::user::UserServer;
use rmcs_api_server::auth::profile::ProfileServer;
use rmcs_api_server::auth::token::TokenServer;
use rmcs_api_server::auth::auth::AuthServer;
use rmcs_api_server::resource::model::ModelServer;
use rmcs_api_server::resource::device::DeviceServer;
use rmcs_api_server::resource::group::GroupServer;
use rmcs_api_server::resource::set::SetServer;
use rmcs_api_server::resource::data::DataServer;
use rmcs_api_server::resource::buffer::BufferServer;
use rmcs_api_server::resource::slice::SliceServer;
use rmcs_api_server::resource::log::LogServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url_auth = std::env::var("DATABASE_AUTH_URL").unwrap();
    let url_resource = std::env::var("DATABASE_RESOURCE_URL").unwrap();
    let addr = std::env::var("ADDRESS").unwrap().parse()?;

    let auth_db = Auth::new_with_url(&url_auth).await;
    let api_server = ApiServer::new(auth_db.clone());
    let role_server = RoleServer::new(auth_db.clone());
    let user_server = UserServer::new(auth_db.clone());
    let profile_server = ProfileServer::new(auth_db.clone());
    let token_server = TokenServer::new(auth_db.clone());
    let auth_server = AuthServer::new(auth_db.clone());

    let resource_db = Resource::new_with_url(&url_resource).await;
    let model_server = ModelServer::new(resource_db.clone());
    let device_server = DeviceServer::new(resource_db.clone());
    let group_server = GroupServer::new(resource_db.clone());
    let set_server = SetServer::new(resource_db.clone());
    let data_server = DataServer::new(resource_db.clone());
    let buffer_server = BufferServer::new(resource_db.clone());
    let slice_server = SliceServer::new(resource_db.clone());
    let log_server = LogServer::new(resource_db.clone());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(auth_descriptor::api::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(auth_descriptor::role::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(auth_descriptor::user::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(auth_descriptor::profile::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(auth_descriptor::token::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(auth_descriptor::auth::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::model::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::device::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::group::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::set::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::data::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::buffer::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::slice::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(resource_descriptor::log::DESCRIPTOR_SET)
        .build();

    tonic::transport::Server::builder()
        .add_service(ApiServiceServer::new(api_server))
        .add_service(RoleServiceServer::new(role_server))
        .add_service(UserServiceServer::new(user_server))
        .add_service(ProfileServiceServer::new(profile_server))
        .add_service(TokenServiceServer::new(token_server))
        .add_service(AuthServiceServer::new(auth_server))
        .add_service(ModelServiceServer::new(model_server))
        .add_service(DeviceServiceServer::new(device_server))
        .add_service(GroupServiceServer::new(group_server))
        .add_service(SetServiceServer::new(set_server))
        .add_service(DataServiceServer::new(data_server))
        .add_service(BufferServiceServer::new(buffer_server))
        .add_service(SliceServiceServer::new(slice_server))
        .add_service(LogServiceServer::new(log_server))
        .add_service(reflection_service?)
        .serve(addr)
        .await?;

    Ok(())
}
