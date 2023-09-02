use tonic::{Request, transport::Channel};
use uuid::Uuid;
use rmcs_auth_api::auth::auth_service_client::AuthServiceClient;
use rmcs_auth_api::auth::{
    ApiKeyRequest, ApiLoginRequest, ApiLoginResponse
};
use super::{generate_transport_keys, export_public_key, import_public_key, encrypt_message, decrypt_message};

pub async fn api_login(addr: &str, api_id: Uuid, password: &str)
    -> Option<ApiLoginResponse>
{
    let channel = Channel::from_shared(addr.to_owned())
        .expect("Invalid address")
        .connect()
        .await
        .expect(&format!("Error making channel to {}", addr));
    let mut client = AuthServiceClient::new(channel.to_owned());
    let request = Request::new(ApiKeyRequest {
    });
    // get transport public key of requested API and encrypt the password
    let response = client.api_login_key(request).await.ok()?.into_inner();
    let pub_key = import_public_key(response.public_key.as_slice()).ok()?;
    let passhash = encrypt_message(password.as_bytes(), pub_key).ok()?;
    // request API key and procedures access from server
    let (priv_key, pub_key) = generate_transport_keys().ok()?;
    let pub_der = export_public_key(pub_key).ok()?;
    let request = Request::new(ApiLoginRequest {
        api_id: api_id.as_bytes().to_vec(),
        password: passhash,
        public_key: pub_der
    });
    let mut response = client.api_login(request).await.ok()?.into_inner();
    response.root_key = decrypt_message(&response.root_key, priv_key.clone()).ok()?;
    response.access_key = decrypt_message(&response.access_key, priv_key).ok()?;
    Some(response)
}
