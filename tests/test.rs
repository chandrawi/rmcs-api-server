#[cfg(test)]
mod tests {
    use rmcs_api_server::utility::{import_public_key, encrypt_message};
    use rmcs_auth_api::auth::auth_service_client::AuthServiceClient;
    use rmcs_auth_api::auth::{
        UserKeyRequest, UserLoginRequest, UserRefreshRequest
    };
    use tonic::Request;

    #[tokio::test]
    async fn test_auth() -> Result<(), Box<dyn std::error::Error>>
    {
        dotenvy::dotenv().ok();
        let addr = std::env::var("ADDRESS_AUTH").unwrap();
        let client_addr = String::from("http://") + &addr;

        let mut auth = AuthServiceClient::connect(client_addr).await.unwrap();

        let username = "administrator";
        let request = Request::new(UserKeyRequest {
            name: username.to_owned()
        });
        let response = auth.user_login_key(request).await.unwrap().into_inner();
        let pub_key = import_public_key(&response.public_key).unwrap();

        let password = "Adm1n_P4s5w0rd";
        let password_hash = encrypt_message(password.as_bytes(), pub_key).unwrap();
        let request = Request::new(UserLoginRequest {
            name: username.to_owned(),
            password: password_hash
        });
        let response = auth.user_login(request).await.unwrap().into_inner();
        let refresh_token1 = response.refresh_token.clone();
        let access_token_map = response.access_tokens.into_iter().next().unwrap();
        let (api_id1, access_token1) = (access_token_map.api_id, access_token_map.access_token.clone());

        std::thread::sleep(std::time::Duration::from_secs(1));

        let request = Request::new(UserRefreshRequest {
            refresh_token: refresh_token1.clone(),
            access_token: Some(access_token_map)
        });
        let response = auth.user_refresh(request).await.unwrap().into_inner();
        let refresh_token2 = response.refresh_token.clone();
        let (api_id2, access_token2) = response.access_token
            .map(|s| (s.api_id, s.access_token))
            .unwrap();

        assert_eq!(api_id1, api_id2);
        assert_ne!(access_token1, access_token2);
        assert_ne!(refresh_token1, refresh_token2);

        Ok(())
    }

}
