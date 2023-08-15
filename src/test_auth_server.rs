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
use rmcs_api_server::utility::interceptor;
use rmcs_api_server::utility::validator::AuthValidator;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    db_url: Option<String>,
    #[arg(long)]
    address: Option<String>,
    #[arg(long)]
    secured: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    let db_url = match args.db_url {
        Some(value) => value,
        None => std::env::var("DATABASE_URL_AUTH_TEST").unwrap()
    };
    let address = match args.address {
        Some(value) => value,
        None => std::env::var("BIND_ADDRESS_AUTH").unwrap()
    };

    if args.secured {
        auth_server_secured(db_url, address).await
    } else {
        auth_server(db_url, address).await
    }
}

async fn auth_server(db_url: String, address: String) -> Result<(), Box<dyn std::error::Error>>
{
    let addr = address.parse()?;

    let auth_db = Auth::new_with_url(&db_url).await;
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

async fn auth_server_secured(db_url: String, address: String) -> Result<(), Box<dyn std::error::Error>>
{
    let addr = address.parse()?;

    let auth_db = Auth::new_with_url(&db_url).await;
    let api_server = ApiServer::new(auth_db.clone()).with_validator();
    let role_server = RoleServer::new(auth_db.clone()).with_validator();
    let user_server = UserServer::new(auth_db.clone()).with_validator();
    let token_server = TokenServer::new(auth_db.clone()).with_validator();
    let auth_server = AuthServer::new(auth_db.clone());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(descriptor::api::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::role::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::user::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::token::DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(descriptor::auth::DESCRIPTOR_SET)
        .build();

    tonic::transport::Server::builder()
        .add_service(ApiServiceServer::with_interceptor(api_server, interceptor))
        .add_service(RoleServiceServer::with_interceptor(role_server, interceptor))
        .add_service(UserServiceServer::with_interceptor(user_server, interceptor))
        .add_service(TokenServiceServer::with_interceptor(token_server, interceptor))
        .add_service(AuthServiceServer::new(auth_server))
        .add_service(reflection_service?)
        .serve(addr)
        .await?;

    Ok(())
}
