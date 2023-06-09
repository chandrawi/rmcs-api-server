use rmcs_auth_db::Auth;
use rmcs_auth_api::api::api_service_server::ApiServiceServer;
use rmcs_auth_api::role::role_service_server::RoleServiceServer;
use rmcs_auth_api::user::user_service_server::UserServiceServer;
use rmcs_auth_api::token::token_service_server::TokenServiceServer;
use rmcs_auth_api::auth::auth_service_server::AuthServiceServer;
use rmcs_auth_api::descriptor;
use rmcs_api_server::auth::api::ApiServer;
use rmcs_api_server::auth::role::RoleServer;
use rmcs_api_server::auth::user::UserServer;
use rmcs_api_server::auth::token::TokenServer;
use rmcs_api_server::auth::auth::AuthServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_AUTH_URL").unwrap();
    let addr = std::env::var("ADDRESS_AUTH").unwrap().parse()?;

    let auth_db = Auth::new_with_url(&url).await;
    let api_server = ApiServer::new(auth_db.clone());
    let role_server = RoleServer::new(auth_db.clone());
    let user_server = UserServer::new(auth_db.clone());
    let token_server = TokenServer::new(auth_db.clone());
    let auth_server = AuthServer::new(auth_db.clone());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(descriptor::api::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::role::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::user::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::token::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::auth::DESCRIPTOR_SET)
        .build();

    tonic::transport::Server::builder()
        .add_service(ApiServiceServer::new(api_server))
        .add_service(RoleServiceServer::new(role_server))
        .add_service(UserServiceServer::new(user_server))
        .add_service(TokenServiceServer::new(token_server))
        .add_service(AuthServiceServer::new(auth_server))
        .add_service(reflection_service?)
        .serve(addr)
        .await?;

    Ok(())
}
