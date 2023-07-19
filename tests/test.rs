#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::env;
    use tonic::{Request, Status, transport::{Server, Channel}, service::Interceptor, metadata::MetadataValue};
    use std::process::{Command, Stdio};
    use std::time::{SystemTime, Duration};
    use rmcs_auth_db::Auth;
    use rmcs_auth_api::api::{api_service_server::ApiServiceServer, api_service_client::ApiServiceClient};
    use rmcs_auth_api::api::{ApiSchema, ProcedureSchema, ApiId, ProcedureId};
    use rmcs_auth_api::role::{role_service_server::RoleServiceServer, role_service_client::RoleServiceClient};
    use rmcs_auth_api::role::{RoleSchema, RoleAccess, RoleId};
    use rmcs_auth_api::user::{user_service_server::UserServiceServer, user_service_client::UserServiceClient};
    use rmcs_auth_api::user::{UserSchema, UserRole, UserId};
    use rmcs_auth_api::token::token_service_server::TokenServiceServer;
    use rmcs_auth_api::auth::{auth_service_server::AuthServiceServer, auth_service_client::AuthServiceClient};
    use rmcs_auth_api::auth::{UserKeyRequest, UserLoginRequest, UserLogoutRequest};
    use rmcs_resource_db::Resource;
    use rmcs_resource_api::model::{model_service_server::ModelServiceServer, model_service_client::ModelServiceClient};
    use rmcs_resource_api::model::{ModelSchema, ModelTypes, ModelId};
    use rmcs_resource_api::device::device_service_server::DeviceServiceServer;
    use rmcs_resource_api::group::group_service_server::GroupServiceServer;
    use rmcs_resource_api::data::data_service_server::DataServiceServer;
    use rmcs_resource_api::buffer::buffer_service_server::BufferServiceServer;
    use rmcs_resource_api::slice::slice_service_server::SliceServiceServer;
    use rmcs_resource_api::log::log_service_server::LogServiceServer;
    use rmcs_api_server::auth::api::ApiServer;
    use rmcs_api_server::auth::role::RoleServer;
    use rmcs_api_server::auth::user::UserServer;
    use rmcs_api_server::auth::token::TokenServer;
    use rmcs_api_server::auth::auth::AuthServer;
    use rmcs_api_server::resource::model::ModelServer;
    use rmcs_api_server::resource::device::DeviceServer;
    use rmcs_api_server::resource::group::GroupServer;
    use rmcs_api_server::resource::data::DataServer;
    use rmcs_api_server::resource::buffer::BufferServer;
    use rmcs_api_server::resource::slice::SliceServer;
    use rmcs_api_server::resource::log::LogServer;
    use rmcs_api_server::utility::interceptor;
    use rmcs_api_server::utility::validator::{AuthValidator, AccessValidator, AccessSchema};
    use rmcs_api_server::utility::{import_public_key, encrypt_message};
    use rmcs_api_server::utility::root::{ROOT_NAME, root_data};

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

    async fn auth_server(db_url: &str, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let auth_db = Auth::new_with_url(db_url).await;
        let api_server = ApiServer::new(auth_db.clone()).with_validator();
        let role_server = RoleServer::new(auth_db.clone()).with_validator();
        let user_server = UserServer::new(auth_db.clone()).with_validator();
        let token_server = TokenServer::new(auth_db.clone()).with_validator();
        let auth_server = AuthServer::new(auth_db.clone());
    
        Server::builder()
            .add_service(ApiServiceServer::with_interceptor(api_server, interceptor))
            .add_service(RoleServiceServer::with_interceptor(role_server, interceptor))
            .add_service(UserServiceServer::with_interceptor(user_server, interceptor))
            .add_service(TokenServiceServer::with_interceptor(token_server, interceptor))
            .add_service(AuthServiceServer::new(auth_server))
            .serve(addr.parse()?)
            .await?;
    
        Ok(())
    }

    async fn resource_server(db_url: &str, addr: &str, token_key: &[u8], accesses: &[AccessSchema]) -> Result<(), Box<dyn std::error::Error>> {
        let resource_db = Resource::new_with_url(db_url).await;
        let model_server = ModelServer::new(resource_db.clone())
            .with_validator(token_key, accesses);
        let device_server = DeviceServer::new(resource_db.clone())
            .with_validator(token_key, accesses);
        let group_server = GroupServer::new(resource_db.clone())
            .with_validator(token_key, accesses);
        let data_server = DataServer::new(resource_db.clone())
            .with_validator(token_key, accesses);
        let buffer_server = BufferServer::new(resource_db.clone())
            .with_validator(token_key, accesses);
        let slice_server = SliceServer::new(resource_db.clone())
            .with_validator(token_key, accesses);
        let log_server = LogServer::new(resource_db.clone())
            .with_validator(token_key, accesses);
    
        Server::builder()
            .add_service(ModelServiceServer::with_interceptor(model_server, interceptor))
            .add_service(DeviceServiceServer::with_interceptor(device_server, interceptor))
            .add_service(GroupServiceServer::with_interceptor(group_server, interceptor))
            .add_service(DataServiceServer::with_interceptor(data_server, interceptor))
            .add_service(BufferServiceServer::with_interceptor(buffer_server, interceptor))
            .add_service(SliceServiceServer::with_interceptor(slice_server, interceptor))
            .add_service(LogServiceServer::with_interceptor(log_server, interceptor))
            .serve(addr.parse()?)
            .await?;
    
        Ok(())
    }

    fn wait_server(port: &str) {
        let mut count = 0;
        let time_limit = SystemTime::now() + Duration::from_secs(30);
        while SystemTime::now() < time_limit && count == 0 {
            let ss_child = Command::new("ss")
                .arg("-tulpn")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let grep_child = Command::new("grep")
                .args([port, "-c"])
                .stdin(Stdio::from(ss_child.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let output = grep_child.wait_with_output().unwrap();
            count = String::from_utf8(output.stdout)
                .unwrap()
                .replace("\n", "")
                .parse()
                .unwrap_or(0);
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    #[derive(Clone)]
    pub(crate) struct TokenInterceptor(pub(crate) String);
    
    impl Interceptor for TokenInterceptor {
        fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
            request.metadata_mut().insert(
                "authorization", 
                MetadataValue::try_from(String::from("Bearer ") + &self.0).unwrap()
            );
            Ok(request)
        }
    }

    async fn login(address: &str, username: &str, password: &str) -> (u32, String, String, String) {
        let channel = Channel::from_shared(address.to_owned()).unwrap().connect().await.unwrap();
        let mut client = AuthServiceClient::new(channel);
        let request = Request::new(UserKeyRequest {
            username: username.to_owned()
        });
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
        (response.user_id, response.auth_token, access_token, refresh_token)
    }

    async fn logout(address: &str, user_id: u32, auth_token: &str) {
        let channel = Channel::from_shared(address.to_owned()).unwrap().connect().await.unwrap();
        let mut client = AuthServiceClient::new(channel);
        let request = Request::new(UserLogoutRequest {
            user_id,
            auth_token: auth_token.to_owned()
        });
        client.user_logout(request).await.unwrap().into_inner();
    }

    #[tokio::test]
    async fn test_auth() -> Result<(), Box<dyn std::error::Error>>
    {
        // get database urls and server address for auth and resource
        dotenvy::dotenv().ok();
        let auth_db_url = env::var("DATABASE_AUTH_TEST_URL").unwrap();
        let auth_addr = env::var("ADDRESS_AUTH").unwrap();
        let auth_addr_client = String::from("http://") + &auth_addr;
        let auth_port = auth_addr_client.split(":").into_iter().last().unwrap();
        let resource_db_url = env::var("DATABASE_RESOURCE_TEST_URL").unwrap();
        let resource_addr = env::var("ADDRESS_RESOURCE").unwrap();
        let resource_addr_client = String::from("http://") + &resource_addr;
        let resource_port = resource_addr_client.split(":").into_iter().last().unwrap();

        // start auth server and wait until server process is running
        tokio::spawn(async move {
            auth_server(&auth_db_url, &auth_addr).await.unwrap();
            tokio::task::yield_now().await;
        });
        wait_server(auth_port);

        // root login
        let root = root_data().unwrap();
        let (root_id, root_auth, _, _) = 
            login(&auth_addr_client, ROOT_NAME, &root.password).await;

        // construct api, role, and user service using root token
        let channel = Channel::from_shared(auth_addr_client.clone()).unwrap().connect().await.unwrap();
        let interceptor = TokenInterceptor(root_auth.to_owned());
        let mut api_service = 
            ApiServiceClient::with_interceptor(channel.clone(), interceptor.clone());
        let mut role_service = 
            RoleServiceClient::with_interceptor(channel.clone(), interceptor.clone());
        let mut user_service = 
            UserServiceClient::with_interceptor(channel.clone(), interceptor.clone());

        // create api and procedures
        let request = Request::new(ApiSchema {
            id: 0,
            name: String::from("resource api"),
            address: resource_addr_client.clone(),
            category: String::from("resource"),
            description: String::new(),
            public_key: Vec::new(),
            password: String::from("Api_pa55w0rd"),
            access_key: Vec::new(),
            procedures: Vec::new()
        });
        let response = api_service.create_api(request).await.unwrap().into_inner();
        let api_id = response.id;
        let proc_names: Vec<String> = ACCESSES.iter().map(|&i| i.0.to_owned()).collect();
        let mut proc_ids = Vec::new();
        for name in &proc_names {
            let request = Request::new(ProcedureSchema {
                id: 0,
                api_id,
                name: name.to_owned(),
                description: String::new(),
                roles: Vec::new()
            });
            let response = api_service.create_procedure(request).await.unwrap().into_inner();
            proc_ids.push(response.id);
        };

        // create roles and link it to procedures
        let mut role_ids = Vec::new();
        for &name in ROLES {
            let request = Request::new(RoleSchema {
                id: 0,
                api_id,
                name: name.to_owned(),
                multi: false,
                ip_lock: true,
                access_duration: 900,
                refresh_duration: 43200,
                access_key: Vec::new(),
                procedures: Vec::new()
            });
            let response = role_service.create_role(request).await.unwrap().into_inner();
            role_ids.push(response.id);
        }
        let mut role_accesses = Vec::new();
        for access in ACCESSES {
            let index = proc_names.iter()
                .enumerate()
                .filter(|(_, s)| **s == access.0)
                .map(|(i, _)| i)
                .next().unwrap();
            let proc_id = proc_ids.get(index).unwrap().to_owned();
            for &role in access.1 {
                let index = ROLES.iter()
                    .enumerate()
                    .filter(|(_, s)| **s == role)
                    .map(|(i, _)| i)
                    .next().unwrap();
                let role_id = role_ids.get(index).unwrap().to_owned();
                role_accesses.push((role_id, proc_id));
            }
        }
        for &(id, procedure_id) in &role_accesses {
            let request = Request::new(RoleAccess {
                id,
                procedure_id
            });
            role_service.add_role_access(request).await.unwrap();
        }

        // create users and link it to a role
        let mut user_ids = Vec::new();
        let mut user_roles = Vec::new();
        for &(user, password, role) in USERS {
            let request = Request::new(UserSchema {
                id: 0,
                name: user.to_owned(),
                email: String::new(),
                phone: String::new(),
                public_key: Vec::new(),
                password: password.to_owned(),
                roles: Vec::new()
            });
            let response = user_service.create_user(request).await.unwrap().into_inner();
            user_ids.push(response.id);
            let index = ROLES.iter()
                .enumerate()
                .filter(|(_, s)| **s == role)
                .map(|(i, _)| i)
                .next().unwrap();
            let role_id = role_ids.get(index).unwrap().to_owned();
            user_roles.push((response.id, role_id));
        }
        for &(user_id, role_id) in &user_roles {
            let request = Request::new(UserRole {
                user_id,
                role_id
            });
            user_service.add_user_role(request).await.unwrap();
        }

        // get api access key and role access schema to be used by resource server
        let request = Request::new(ApiId {
            id: api_id
        });
        let response = api_service.read_api(request).await.unwrap().into_inner();
        let api_schema = response.result.unwrap();
        let token_key = api_schema.access_key;
        let accesses: Vec<AccessSchema> = api_schema.procedures.into_iter().map(|s| {
            AccessSchema {
                procedure: s.name,
                roles: s.roles
            }
        }).collect();

        // start resource server and wait until server process is running
        tokio::spawn(async move {
            resource_server(&resource_db_url, &resource_addr, &token_key, &accesses).await.unwrap();
            tokio::task::yield_now().await;
        });
        wait_server(resource_port);

        // user and admin login
        let (user_id, user_auth, user_access, _) = 
            login(&auth_addr_client, USER_NAME, USER_PW).await;
        let (admin_id, admin_auth, admin_access, _) = 
            login(&auth_addr_client, ADMIN_NAME, ADMIN_PW).await;

        // construct model service for admin and user
        let channel = Channel::from_shared(resource_addr_client.clone()).unwrap().connect().await.unwrap();
        let interceptor_user = TokenInterceptor(user_access.to_owned());
        let interceptor_admin = TokenInterceptor(admin_access.to_owned());
        let mut model_service_user = 
            ModelServiceClient::with_interceptor(channel.clone(), interceptor_user.clone());
        let mut model_service_admin = 
            ModelServiceClient::with_interceptor(channel.clone(), interceptor_admin.clone());

        // try to create model using user service and admin service, user should failed and admin should success
        let schema = ModelSchema {
            id: 0,
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
            id: model_id,
            types: vec![2, 6]
        });
        model_service_admin.add_model_type(request).await.unwrap();

        // read created model using user service
        let request = Request::new(ModelId {
            id: model_id
        });
        let try_response = model_service_user.read_model(request).await;
        assert!(try_response.is_ok());

        // remove model type and delete model
        let id = ModelId {
            id: model_id
        };
        let request = Request::new(id.clone());
        model_service_admin.remove_model_type(request).await.unwrap();
        let request = Request::new(id.clone());
        let try_response = model_service_admin.delete_model(request).await;
        assert!(try_response.is_ok());

        // user and admin logout
        logout(&auth_addr_client, user_id, &user_auth).await;
        logout(&auth_addr_client, admin_id, &admin_auth).await;

        // remove user links to role and delete user
        for &(user_id, role_id) in &user_roles {
            let request = Request::new(UserRole {
                user_id,
                role_id
            });
            user_service.remove_user_role(request).await.unwrap();
        }
        for &id in &user_ids {
            let request = Request::new(UserId {
                id
            });
            user_service.delete_user(request).await.unwrap();
        }

        // remove role links to procedure and delete roles
        for &(id, procedure_id) in &role_accesses {
            let request = Request::new(RoleAccess {
                id,
                procedure_id
            });
            role_service.remove_role_access(request).await.unwrap();
        }
        for &id in &role_ids {
            let request = Request::new(RoleId {
                id
            });
            role_service.delete_role(request).await.unwrap();
        }

        // delete procedures and api
        for &id in &proc_ids {
            let request = Request::new(ProcedureId {
                id
            });
            api_service.delete_procedure(request).await.unwrap();
        }
        let request = Request::new(ApiId {
            id: api_id
        });
        api_service.delete_api(request).await.unwrap();

        // root logout
        logout(&auth_addr_client, root_id, &root_auth).await;

        // try to read api after logout, should error
        let request = Request::new(ApiId {
            id: api_id
        });
        let try_response = api_service.read_api(request).await;
        assert!(try_response.is_err());

        Ok(())
    }

}
