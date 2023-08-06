use std::path::PathBuf;
use std::sync::OnceLock;
use uuid::Uuid;
use rsa::{RsaPrivateKey, RsaPublicKey};
use pkcs8::{DecodePrivateKey, EncodePrivateKey};
use spki::{DecodePublicKey, EncodePublicKey};
use rmcs_auth_db::schema::auth_user::{UserSchema, UserRoleSchema};

pub const ROOT_ID: Uuid = Uuid::from_u128(0xffffffffffffffffffffffffffffffffu128);
pub const ROOT_NAME: &str = "root";
pub static ROOT_DATA: OnceLock<RootData> = OnceLock::new();

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

impl Into<UserSchema> for RootData {
    fn into(self) -> UserSchema {
        UserSchema {
            id: ROOT_ID,
            name: ROOT_NAME.to_owned(),
            email: String::new(),
            phone: String::new(),
            public_key: root_public_key().unwrap_or_default(),
            private_key: root_private_key().unwrap_or_default(),
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

pub fn root_private_key() -> Option<Vec<u8>>
{
    let root_priv = PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "root", "private.pem"]);
    let priv_pem = std::fs::read_to_string(root_priv).ok()?;
    let priv_der = RsaPrivateKey::from_pkcs8_pem(&priv_pem).ok()?
        .to_pkcs8_der().ok()?
        .to_bytes()
        .to_vec();
    Some(priv_der)
}

pub(crate) fn root_public_key() -> Option<Vec<u8>>
{
    let root_priv = PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "root", "public.pem"]);
    let pub_pem = std::fs::read_to_string(root_priv).ok()?;
    let pub_der = RsaPublicKey::from_public_key_pem(&pub_pem).ok()?
        .to_public_key_der().ok()?
        .to_vec();
    Some(pub_der)
}
