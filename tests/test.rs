#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use tonic::{Request, transport::Channel};
    use uuid::Uuid;
    use rmcs_auth_api::api::api_service_client::ApiServiceClient;
    use rmcs_auth_api::api::{ApiSchema, ProcedureSchema, ApiId, ProcedureId};
    use rmcs_auth_api::role::role_service_client::RoleServiceClient;
    use rmcs_auth_api::role::{RoleSchema, RoleAccess, RoleId};
    use rmcs_auth_api::user::user_service_client::UserServiceClient;
    use rmcs_auth_api::user::{UserSchema, UserRole, UserId};
    use rmcs_auth_api::auth::auth_service_client::AuthServiceClient;
    use rmcs_auth_api::auth::{UserKeyRequest, UserLoginRequest, UserRefreshRequest, UserLogoutRequest};
    use rmcs_resource_api::model::model_service_client::ModelServiceClient;
    use rmcs_resource_api::model::{ModelSchema, ModelTypes, ModelId};
    use rmcs_api_server::utility::{import_public_key, encrypt_message};
    use rmcs_api_server::utility::config::{ROOT_NAME, ROOT_DATA};
    use rmcs_api_server::utility::interceptor::TokenInterceptor;
    use rmcs_api_client::utility::test::{TestServerKind, TestServer};

    const ACCESSES: &[(&str, &[&str])] = &[
        ("read_model", &["admin", "user"]),
        ("create_model", &["admin"]),
        ("delete_model", &["admin"]),
        ("add_model_type", &["admin"]),
        ("remove_model_type", &["admin"])
    ];

    const ROLES: &[&str] = &["admin", "user"];
    
    const ADMIN_NAME: &str = "administrator";
    const USER_NAME: &str = "username";
    const ADMIN_PW: &str = "Adm1n_P4s5w0rd";
    const USER_PW: &str = "Us3r_P4s5w0rd";
    
    const USERS: &[(&str, &str, &str)] = &[
        (ADMIN_NAME, ADMIN_PW, "admin"),
        (USER_NAME, USER_PW, "user")
    ];

    async fn login(address: &str, username: &str, password: &str) -> (Uuid, String, String, String) {
        let channel = Channel::from_shared(address.to_owned()).unwrap().connect().await.unwrap();
        let mut client = AuthServiceClient::new(channel);
        let request = Request::new(UserKeyRequest { });
        // get transport public key of requested user and encrypt the password
        let response = client.user_login_key(request).await.unwrap().into_inner();
        let pub_key = import_public_key(response.public_key.as_slice()).unwrap();
        let passhash = encrypt_message(password.as_bytes(), pub_key).unwrap();
        // request access and refresh tokens
        let request = Request::new(UserLoginRequest {
            username: username.to_owned(),
            password: passhash
        });
        let response = client.user_login(request).await.unwrap().into_inner();
        let (access_token, refresh_token) = response.access_tokens.into_iter()
            .map(|a| (a.access_token, a.refresh_token)).next().unwrap();
        (Uuid::from_slice(&response.user_id).unwrap(), response.auth_token, access_token, refresh_token)
    }

    async fn refresh(address: &str, api_id: Uuid, access_token: &str, refresh_token: &str) -> (String, String) {
        let channel = Channel::from_shared(address.to_owned()).unwrap().connect().await.unwrap();
        let mut client = AuthServiceClient::new(channel);
        let request = Request::new(UserRefreshRequest {
            api_id: api_id.as_bytes().to_vec(),
            access_token: access_token.to_owned(),
            refresh_token: refresh_token.to_owned()
        });
        let response = client.user_refresh(request).await.unwrap().into_inner();
        (response.access_token, response.refresh_token)
    }

    async fn logout(address: &str, user_id: Uuid, auth_token: &str) {
        let channel = Channel::from_shared(address.to_owned()).unwrap().connect().await.unwrap();
        let mut client = AuthServiceClient::new(channel);
        let request = Request::new(UserLogoutRequest {
            user_id: user_id.as_bytes().to_vec(),
            auth_token: auth_token.to_owned()
        });
        client.user_logout(request).await.unwrap().into_inner();
    }

    #[tokio::test]
    async fn test_auth() -> Result<(), Box<dyn std::error::Error>>
    {
        // start auth server and wait until server process is running
        let auth_server = TestServer::new_secured(TestServerKind::Auth, None, None);
        auth_server.truncate_tables().await.unwrap();
        auth_server.start_server();

        // root login
        let root = ROOT_DATA.get().map(|x| x.to_owned()).unwrap_or_default();
        let (root_id, root_auth, _, _) = 
            login(&auth_server.address, ROOT_NAME, &root.password).await;

        // construct api, role, and user service using root token
        let channel = Channel::from_shared(auth_server.address.clone()).unwrap().connect().await.unwrap();
        let interceptor = TokenInterceptor(root_auth.to_owned());
        let mut api_service = 
            ApiServiceClient::with_interceptor(channel.clone(), interceptor.clone());
        let mut role_service = 
            RoleServiceClient::with_interceptor(channel.clone(), interceptor.clone());
        let mut user_service = 
            UserServiceClient::with_interceptor(channel.clone(), interceptor.clone());

        // create api and procedures
        let password = String::from("Api_pa55w0rd");
        let request = Request::new(ApiSchema {
            id: Uuid::nil().as_bytes().to_vec(),
            name: String::from("resource api"),
            address: String::new(),
            category: String::from("resource"),
            description: String::new(),
            password: password.clone(),
            access_key: Vec::new(),
            procedures: Vec::new()
        });
        let response = api_service.create_api(request).await.unwrap().into_inner();
        let api_id = response.id;
        let mut proc_map = Vec::new();
        for &(name, _) in ACCESSES {
            let request = Request::new(ProcedureSchema {
                id: Uuid::nil().as_bytes().to_vec(),
                api_id: api_id.clone(),
                name: name.to_owned(),
                description: String::new(),
                roles: Vec::new()
            });
            let response = api_service.create_procedure(request).await.unwrap().into_inner();
            proc_map.push((response.id, name))
        };

        // create roles and link it to procedures
        let mut role_map = Vec::new();
        for &name in ROLES {
            let request = Request::new(RoleSchema {
                id: Uuid::nil().as_bytes().to_vec(),
                api_id: api_id.clone(),
                name: name.to_owned(),
                multi: false,
                ip_lock: true,
                access_duration: 900,
                refresh_duration: 43200,
                access_key: Vec::new(),
                procedures: Vec::new()
            });
            let response = role_service.create_role(request).await.unwrap().into_inner();
            role_map.push((response.id, name));
        }
        let mut role_accesses = Vec::new();
        for &(proc_name, roles) in ACCESSES {
            let proc_id = proc_map.iter()
                .filter(|(_, s)| *s == proc_name)
                .map(|(i, _)| i)
                .next().unwrap();
            for &role in roles {
                let role_id = role_map.iter()
                    .filter(|(_, s)| *s == role)
                    .map(|(i, _)| i)
                    .next().unwrap();
                role_accesses.push((role_id.to_owned(), proc_id.to_owned()));
            }
        }
        for (id, procedure_id) in role_accesses.clone() {
            let request = Request::new(RoleAccess {
                id,
                procedure_id
            });
            role_service.add_role_access(request).await.unwrap();
        }

        // create users and link it to a role
        let mut user_roles = Vec::new();
        for &(user, password, role) in USERS {
            let request = Request::new(UserSchema {
                id: Uuid::nil().as_bytes().to_vec(),
                name: user.to_owned(),
                email: String::new(),
                phone: String::new(),
                password: password.to_owned(),
                roles: Vec::new()
            });
            let response = user_service.create_user(request).await.unwrap().into_inner();
            let role_id = role_map.iter()
                .filter(|(_, s)| *s == role)
                .map(|(i, _)| i)
                .next().unwrap();
            user_roles.push((response.id.clone(), role_id.to_owned()));
        }
        for (user_id, role_id) in user_roles.clone() {
            let request = Request::new(UserRole {
                user_id,
                role_id
            });
            user_service.add_user_role(request).await.unwrap();
        }

        // start resource server and wait until server process is running
        let resource_api_id = Uuid::from_slice(&api_id).unwrap().to_string();
        let resource_server = TestServer::new_secured(TestServerKind::Resource, Some(&resource_api_id), Some(&password));
        resource_server.truncate_tables().await.unwrap();
        resource_server.start_server();

        // user and admin login
        let (user_id, user_auth, user_access, user_refresh) = 
            login(&auth_server.address, USER_NAME, USER_PW).await;
        let (admin_id, admin_auth, admin_access, _) = 
            login(&auth_server.address, ADMIN_NAME, ADMIN_PW).await;

        // construct model service for admin and user
        let channel = Channel::from_shared(resource_server.address.clone()).unwrap().connect().await.unwrap();
        let interceptor_user = TokenInterceptor(user_access.to_owned());
        let interceptor_admin = TokenInterceptor(admin_access.to_owned());
        let mut model_service_user = 
            ModelServiceClient::with_interceptor(channel.clone(), interceptor_user.clone());
        let mut model_service_admin = 
            ModelServiceClient::with_interceptor(channel.clone(), interceptor_admin.clone());

        // try to create model using user service and admin service, user should failed and admin should success
        let schema = ModelSchema {
            id: Uuid::nil().as_bytes().to_vec(),
            indexing: 0, // Timestamp indexing
            category: String::from("UPLINK"),
            name: String::from("name"),
            description: String::new(),
            types: vec![1, 1],
            configs: Vec::new()
        };
        let request = Request::new(schema.clone());
        let try_response = model_service_user.create_model(request).await;
        assert!(try_response.is_err());
        let request = Request::new(schema.clone());
        let try_response = model_service_admin.create_model(request).await;
        assert!(try_response.is_ok());

        // add model type using admin service
        let model_id = try_response.unwrap().into_inner().id;
        let request = Request::new(ModelTypes {
            id: model_id.clone(),
            types: vec![2, 6]
        });
        model_service_admin.add_model_type(request).await.unwrap();

        // read created model using user service
        let request = Request::new(ModelId {
            id: model_id.clone()
        });
        let try_response = model_service_user.read_model(request).await;
        assert!(try_response.is_ok());

        // refresh user
        let (user_access, _) = refresh(&auth_server.address, Uuid::from_slice(&api_id).unwrap(), &user_access, &user_refresh).await;
        let interceptor_user = TokenInterceptor(user_access.to_owned());
        let mut model_service_user = 
            ModelServiceClient::with_interceptor(channel.clone(), interceptor_user.clone());
        // try to read model again after refreshing token
        let request = Request::new(ModelId {
            id: model_id.clone()
        });
        model_service_user.read_model(request).await.unwrap();

        // remove model type and delete model
        let id = ModelId {
            id: model_id.clone()
        };
        let request = Request::new(id.clone());
        model_service_admin.remove_model_type(request).await.unwrap();
        let request = Request::new(id.clone());
        let try_response = model_service_admin.delete_model(request).await;
        assert!(try_response.is_ok());

        // user and admin logout
        logout(&auth_server.address, user_id, &user_auth).await;
        logout(&auth_server.address, admin_id, &admin_auth).await;

        // remove user links to role and delete user
        for (user_id, role_id) in user_roles.clone() {
            let request = Request::new(UserRole {
                user_id,
                role_id
            });
            user_service.remove_user_role(request).await.unwrap();
        }
        for (id, _) in user_roles.clone() {
            let request = Request::new(UserId {
                id
            });
            user_service.delete_user(request).await.unwrap();
        }

        // remove role links to procedure and delete roles
        for (id, procedure_id) in role_accesses.clone() {
            let request = Request::new(RoleAccess {
                id,
                procedure_id
            });
            role_service.remove_role_access(request).await.unwrap();
        }
        for (id, _) in role_map.clone() {
            let request = Request::new(RoleId {
                id
            });
            role_service.delete_role(request).await.unwrap();
        }

        // delete procedures and api
        for (id, _) in proc_map.clone() {
            let request = Request::new(ProcedureId {
                id
            });
            api_service.delete_procedure(request).await.unwrap();
        }
        let request = Request::new(ApiId {
            id: api_id.clone()
        });
        api_service.delete_api(request).await.unwrap();

        // root logout
        logout(&auth_server.address, root_id, &root_auth).await;

        // try to read api after logout, should error
        let request = Request::new(ApiId {
            id: api_id.clone()
        });
        let try_response = api_service.read_api(request).await;
        assert!(try_response.is_err());

        // stop servers
        auth_server.stop_server();
        resource_server.stop_server();

        Ok(())
    }

}
