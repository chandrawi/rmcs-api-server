use std::env;
use std::process::{Command, Stdio};
use std::time::{SystemTime, Duration};
use sqlx::Error;
use sqlx::postgres::PgPoolOptions;

pub enum TestServerKind {
    Auth,
    Resource
}

pub struct TestServer {
    pub kind: TestServerKind,
    pub db_url: String,
    pub address: String,
    pub bin_name: String,
    pub secured: bool,
    pub api_cred: Option<(String, String, String)>
}

impl TestServer {

    pub fn new(kind: TestServerKind) -> TestServer
    {
        dotenvy::dotenv().ok();
        let (env_db, env_addr, bin_name) = match kind {
            TestServerKind::Auth => ("DATABASE_URL_AUTH_TEST", "SERVER_ADDRESS_AUTH", "test_auth_server"),
            TestServerKind::Resource => ("DATABASE_URL_RESOURCE_TEST", "SERVER_ADDRESS_RESOURCE", "test_resource_server")
        };
        let db_url = env::var(env_db).unwrap();
        let address = env::var(env_addr).unwrap();
        let scheme = address.split(":").next().unwrap();
        let address = 
            if vec!["http", "https"].contains(&scheme) { address } 
            else { String::from("http://") + address.as_str() };
        let bin_name = String::from(bin_name);
        TestServer { kind, db_url, address, bin_name, secured: false, api_cred: None }
    }

    pub fn new_secured(kind: TestServerKind, api_id: Option<&str>, password: Option<&str>) -> TestServer
    {
        dotenvy::dotenv().ok();
        let auth_address = String::from("http://") + env::var("SERVER_ADDRESS_AUTH").unwrap().as_str();
        let scheme = auth_address.split(":").next().unwrap();
        let auth_address = 
            if vec!["http", "https"].contains(&scheme) { auth_address } 
            else { String::from("http://") + auth_address.as_str() };
        let server = TestServer::new(kind);
        let api_cred = match (api_id, password) {
            (Some(id), Some(pw)) => Some((String::from(id), String::from(pw), auth_address)),
            _ => None
        };
        TestServer { kind: server.kind, db_url: server.db_url, address: server.address, bin_name: server.bin_name, secured: true, api_cred }
    }

    pub async fn truncate_tables(&self) -> Result<(), Error>
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

    pub fn start_server(&self)
    {
        // start server using cargo run command
        let args: Vec<&str> = if self.secured {
            match &self.api_cred {
                Some((id, pw, addr)) => vec![
                    "run", "-p", "rmcs-api-server", "--bin", self.bin_name.as_str(),
                    "--", "--db-url", self.db_url.as_str(), "--secured",
                    "--api-id", id.as_str(), "--password", pw.as_str(),
                    "--auth-address", addr.as_str()
                ],
                None => vec![
                    "run", "-p", "rmcs-api-server", "--bin", self.bin_name.as_str(),
                    "--", "--db-url", self.db_url.as_str(), "--secured"
                ]
            }
        } else {
            vec![
                "run", "-p", "rmcs-api-server", "--bin", self.bin_name.as_str(), "--", "--db-url", self.db_url.as_str()
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

    pub fn stop_server(&self)
    {
        // stop server service
        Command::new("killall")
            .args([self.bin_name.as_str()])
            .spawn()
            .expect("stopping auth server failed");
    }

}
