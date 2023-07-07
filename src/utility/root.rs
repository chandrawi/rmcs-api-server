use std::env;
use base64::{Engine as _, engine::general_purpose};

pub(crate) const ROOT_ID: u32 = 0;
pub(crate) const ROOT_NAME: &str = "root";

pub(crate) struct RootData {
    pub(crate) password: String,
    pub(crate) access_duration: u32,
    pub(crate) refresh_duration: u32,
    pub(crate) access_key: Vec<u8>
}

pub(crate) fn root_data() -> Option<RootData>
{
    dotenvy::dotenv().ok();
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
