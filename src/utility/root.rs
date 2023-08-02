use std::env;
use std::path::PathBuf;
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use rsa::{RsaPrivateKey, RsaPublicKey};
use pkcs8::{DecodePrivateKey, EncodePrivateKey};
use spki::{DecodePublicKey, EncodePublicKey};
use rmcs_auth_db::schema::auth_user::{UserSchema, UserRoleSchema};

pub const ROOT_ID: Uuid = Uuid::from_u128(0xffffffffffffffffffffffffffffffffu128);
pub const ROOT_NAME: &str = "root";

pub struct RootData {
    pub password: String,
    pub access_duration: i32,
    pub refresh_duration: i32,
    pub access_key: Vec<u8>
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
            password: self.password,
            roles: vec![UserRoleSchema {
                api_id: ROOT_ID,
                role: ROOT_NAME.to_owned(),
                multi: false,
                ip_lock: true,
                access_duration: self.access_duration,
                refresh_duration: self.refresh_duration,
                access_key: self.access_key
            }]
        }
    }
}

pub fn root_data() -> Option<RootData>
{
    let root_env = PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "root", ".env"]);
    dotenvy::from_path(root_env).ok()?;
    let password = env::var("ROOT_PASSWORD").ok()?;
    let access_dur = env::var("ROOT_ACCESS_DURATION").ok()?;
    let refresh_dur = env::var("ROOT_REFRESH_DURATION").ok()?;
    let access_key = env::var("ROOT_ACCESS_KEY").ok()?;
    Some(RootData {
        password,
        access_duration: access_dur.parse().ok()?,
        refresh_duration: refresh_dur.parse().ok()?,
        access_key: general_purpose::URL_SAFE_NO_PAD.decode(access_key).ok()?
    })
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
