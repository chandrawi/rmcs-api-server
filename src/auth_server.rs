use rmcs_auth_db::Auth;
use rmcs_auth_db::utility::migrate;
use rmcs_auth_api::api::api_service_server::ApiServiceServer;
use rmcs_auth_api::role::role_service_server::RoleServiceServer;
use rmcs_auth_api::user::user_service_server::UserServiceServer;
use rmcs_auth_api::profile::profile_service_server::ProfileServiceServer;
use rmcs_auth_api::token::token_service_server::TokenServiceServer;
use rmcs_auth_api::auth::auth_service_server::AuthServiceServer;
use rmcs_auth_api::descriptor;
use rmcs_api_server::auth::api::ApiServer;
use rmcs_api_server::auth::role::RoleServer;
use rmcs_api_server::auth::user::UserServer;
use rmcs_api_server::auth::profile::ProfileServer;
use rmcs_api_server::auth::token::TokenServer;
use rmcs_api_server::auth::auth::AuthServer;
use rmcs_api_server::utility::interceptor::interceptor;
use rmcs_api_server::utility::validator::AuthValidator;
use rmcs_api_server::utility::config::{ROOT_DATA, RootData};
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use http::{header::HeaderName, Method};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL_AUTH").unwrap();
    let addr = std::env::var("BIND_ADDRESS_AUTH").unwrap().parse()?;

    let root_pw = std::env::var("ROOT_PASSWORD");
    let root_ad = std::env::var("ROOT_ACCESS_DURATION");
    let root_rd = std::env::var("ROOT_REFRESH_DURATION");
    if let (Ok(password), Ok(access_duration), Ok(refresh_duration)) = (root_pw, root_ad, root_rd) {
        ROOT_DATA.set(RootData::new(
            &password, 
            access_duration.parse()?, 
            refresh_duration.parse()?
        )).unwrap();
    }

    let auth_db = Auth::new_with_url(&url).await;
    migrate(&auth_db.pool).await.unwrap();

    let api_server = ApiServer::new(auth_db.clone()).with_validator();
    let role_server = RoleServer::new(auth_db.clone()).with_validator();
    let user_server = UserServer::new(auth_db.clone()).with_validator();
    let profile_server = ProfileServer::new(auth_db.clone()).with_validator();
    let token_server = TokenServer::new(auth_db.clone()).with_validator();
    let auth_server = AuthServer::new(auth_db.clone());

    let api_server = ApiServiceServer::with_interceptor(api_server, interceptor);
    let role_server = RoleServiceServer::with_interceptor(role_server, interceptor);
    let user_server = UserServiceServer::with_interceptor(user_server, interceptor);
    let profile_server = ProfileServiceServer::with_interceptor(profile_server, interceptor);
    let token_server = TokenServiceServer::with_interceptor(token_server, interceptor);
    let auth_server = AuthServiceServer::new(auth_server);

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(descriptor::api::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::role::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::user::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::profile::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::token::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::auth::DESCRIPTOR_SET)
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
        .add_service(api_server)
        .add_service(role_server)
        .add_service(user_server)
        .add_service(profile_server)
        .add_service(token_server)
        .add_service(auth_server)
        .add_service(reflection_service?)
        .serve(addr)
        .await?;

    Ok(())
}
