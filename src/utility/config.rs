use std::sync::OnceLock;
use uuid::Uuid;
use rsa::RsaPrivateKey;
use super::{generate_transport_keys, export_public_key};
use rmcs_auth_db::schema::auth_user::{UserSchema, UserRoleSchema};
use rmcs_auth_db::utility::generate_access_key;

pub const ROOT_ID: Uuid = Uuid::from_u128(0xffffffffffffffffffffffffffffffffu128);
pub const ROOT_NAME: &str = "root";
pub static ROOT_DATA: OnceLock<RootData> = OnceLock::new();

pub static API_KEY: OnceLock<TransportKey> = OnceLock::new();
pub static USER_KEY: OnceLock<TransportKey> = OnceLock::new();

const DEF_ROOT_PW: &str = "r0ot_P4s5w0rd";
const DEF_ACC_DUR: i32 = 300;
const DEF_REF_DUR: i32 = 3600;
const DEF_ACC_KEY: [u8; 32] = [0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255, 0, 255];

#[derive(Debug, Clone)]
pub struct RootData {
    pub password: String,
    pub access_duration: i32,
    pub refresh_duration: i32,
    pub access_key: Vec<u8>
}

impl Default for RootData {
    fn default() -> Self {
        Self {
            password: String::from(DEF_ROOT_PW),
            access_duration: DEF_ACC_DUR,
            refresh_duration: DEF_REF_DUR,
            access_key: DEF_ACC_KEY.to_vec()
        }
    }
}

impl RootData {
    pub fn new(password: &str, access_duration: i32, refresh_duration: i32) -> Self {
        Self {
            password: String::from(password),
            access_duration,
            refresh_duration,
            access_key: generate_access_key()
        }
    }
    pub fn new_with_key(password: &str, access_duration: i32, refresh_duration: i32, root_key: &[u8]) -> Self {
        Self {
            password: String::from(password),
            access_duration,
            refresh_duration,
            access_key: root_key.to_vec()
        }
    }
}

impl Into<UserSchema> for RootData {
    fn into(self) -> UserSchema {
        UserSchema {
            id: ROOT_ID,
            name: ROOT_NAME.to_owned(),
            email: String::new(),
            phone: String::new(),
            password: String::from(self.password),
            roles: vec![UserRoleSchema {
                api_id: ROOT_ID,
                role: ROOT_NAME.to_owned(),
                multi: false,
                ip_lock: true,
                access_duration: self.access_duration,
                refresh_duration: self.refresh_duration,
                access_key: self.access_key.to_vec()
            }]
        }
    }
}

pub struct TransportKey {
    pub(crate) private_key: RsaPrivateKey,
    pub(crate) public_der: Vec<u8>
}

impl TransportKey {
    pub fn new() -> Self {
        let (private_key, public_key) = generate_transport_keys().unwrap();
        let public_der = export_public_key(public_key).unwrap();
        Self {
            private_key,
            public_der
        }
    }
}
