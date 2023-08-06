#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::env;
    use tonic::{Request, Status, transport::Channel, service::Interceptor, metadata::MetadataValue};
    use std::process::{Command, Stdio};
    use std::time::{SystemTime, Duration};
    use sqlx::Error;
    use sqlx::postgres::PgPoolOptions;
    use uuid::Uuid;
    use rmcs_auth_api::api::api_service_client::ApiServiceClient;
    use rmcs_auth_api::api::{ApiSchema, ProcedureSchema, ApiId, ProcedureId};
    use rmcs_auth_api::role::role_service_client::RoleServiceClient;
    use rmcs_auth_api::role::{RoleSchema, RoleAccess, RoleId};
    use rmcs_auth_api::user::user_service_client::UserServiceClient;
    use rmcs_auth_api::user::{UserSchema, UserRole, UserId};
    use rmcs_auth_api::auth::auth_service_client::AuthServiceClient;
    use rmcs_auth_api::auth::{UserKeyRequest, UserLoginRequest, UserLogoutRequest};
    use rmcs_resource_api::model::model_service_client::ModelServiceClient;
    use rmcs_resource_api::model::{ModelSchema, ModelTypes, ModelId};
    use rmcs_api_server::utility::{import_public_key, encrypt_message};
    use rmcs_api_server::utility::config::{ROOT_NAME, ROOT_DATA};

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

    enum TestServerKind {
        Auth,
        Resource
    }

    struct TestServer {
        kind: TestServerKind,
        db_url: String,
        address: String,
        bin_name: String,
        api_cred: Option<(String, String)>
    }

    impl TestServer {
        fn new(kind: TestServerKind, api_id: Option<&str>, password: Option<&str>) -> TestServer
        {
            dotenvy::dotenv().ok();
            let (env_db, env_addr, bin_name) = match kind {
                TestServerKind::Auth => ("DATABASE_AUTH_TEST_URL", "ADDRESS_AUTH", "test_auth_server"),
                TestServerKind::Resource => ("DATABASE_RESOURCE_TEST_URL", "ADDRESS_RESOURCE", "test_resource_server")
            };
            let db_url = env::var(env_db).unwrap();
            let address = String::from("http://") +  env::var(env_addr).unwrap().as_str();
            let bin_name = String::from(bin_name);
            let api_cred = match (api_id, password) {
                (Some(id), Some(pw)) => Some((String::from(id), String::from(pw))),
                _ => None
            };
            TestServer { kind, db_url, address, bin_name, api_cred }
        }
        async fn truncate_tables(&self) -> Result<(), Error>
        {
            let pool = PgPoolOptions::new().connect(self.db_url.as_str()).await?;
            let sql = match self.kind {
                TestServerKind::Auth => "TRUNCATE TABLE \"token\", \"user_role\", \"user\", \"role_access\", \"role\", \"api_procedure\", \"api\";",
                TestServerKind::Resource => "TRUNCATE TABLE \"system_log\", \"data_slice\", \"data_buffer\", \"data\", \"group_model_map\", \"group_device_map\", \"group_model\", \"group_device\", \"device_config\", \"device\", \"device_type_model\", \"device_type\", \"model_config\", \"model_type\", \"model\";"
            };
            sqlx::query(sql)
                .execute(&pool)
                .await?;
            Ok(())
        }
        fn start_server(&self)
        {
            // start server using cargo run command
            let args: Vec<&str> = match &self.api_cred {
                Some((id, pw)) => vec![
                    "run", "-p", "rmcs-api-server", "--bin", self.bin_name.as_str(),
                    "--", "--db-url", self.db_url.as_str(), "--secured",
                    "--api-id", id.as_str(), "--password", pw.as_str()
                ],
                None => vec![
                    "run", "-p", "rmcs-api-server", "--bin", self.bin_name.as_str(),
                    "--", "--db-url", self.db_url.as_str(), "--secured"
                ]
            };
            Command::new("cargo")
                .args(args)
                .spawn()
                .expect("running auth server failed");
            // wait until server process is running
            let port = String::from(":") + self.address.split(":").into_iter().last().unwrap();
            let mut count = 0;
            let time_limit = SystemTime::now() + Duration::from_secs(30);
            while SystemTime::now() < time_limit && count == 0 {
                let ss_child = Command::new("ss")
                    .arg("-tulpn")
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let grep_child = Command::new("grep")
                    .args([port.as_str(), "-c"])
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
        fn stop_server(&self)
        {
            // stop server service
            Command::new("killall")
                .args([self.bin_name.as_str()])
                .spawn()
                .expect("stopping auth server failed");
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
        let auth_server = TestServer::new(TestServerKind::Auth, None, None);
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
        let proc_names: Vec<String> = ACCESSES.iter().map(|&i| i.0.to_owned()).collect();
        let mut proc_ids = Vec::new();
        for name in &proc_names {
            let request = Request::new(ProcedureSchema {
                id: Uuid::nil().as_bytes().to_vec(),
                api_id: api_id.clone(),
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
                role_accesses.push((role_id, proc_id.clone()));
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
        let mut user_ids = Vec::new();
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
            user_ids.push(response.id.clone());
            let index = ROLES.iter()
                .enumerate()
                .filter(|(_, s)| **s == role)
                .map(|(i, _)| i)
                .next().unwrap();
            let role_id = role_ids.get(index).unwrap().to_owned();
            user_roles.push((response.id.clone(), role_id));
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
        let resource_server = TestServer::new(TestServerKind::Resource, Some(&resource_api_id), Some(&password));
        resource_server.truncate_tables().await.unwrap();
        resource_server.start_server();

        // user and admin login
        let (user_id, user_auth, user_access, _) = 
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
        for id in role_ids.clone() {
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
        for id in role_ids.clone() {
            let request = Request::new(RoleId {
                id
            });
            role_service.delete_role(request).await.unwrap();
        }

        // delete procedures and api
        for id in proc_ids.clone() {
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
