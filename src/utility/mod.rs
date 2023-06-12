use rsa::{RsaPrivateKey, Pkcs1v15Encrypt, RsaPublicKey};
use pkcs8::{DecodePrivateKey, DecodePublicKey};
use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Algorithm, Validation};
use serde::{Serialize, Deserialize};
use rand::thread_rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn import_private_key(priv_der: &[u8]) -> Result<RsaPrivateKey, pkcs8::Error>
{
    let priv_key = RsaPrivateKey::from_pkcs8_der(priv_der)?;
    Ok(priv_key)
}

pub(crate) fn import_public_key(pub_der: &[u8]) -> Result<RsaPublicKey, spki::Error>
{
    let pub_key = RsaPublicKey::from_public_key_der(pub_der)?;
    Ok(pub_key)
}

pub(crate) fn decrypt_message(ciphertext: &[u8], priv_key: RsaPrivateKey) -> Result<Vec<u8>, rsa::Error>
{
    priv_key.decrypt(Pkcs1v15Encrypt, ciphertext)
}

pub(crate) fn encrypt_message(message: &[u8], pub_key: RsaPublicKey) -> Result<Vec<u8>, rsa::Error>
{
    pub_key.encrypt(&mut thread_rng(), Pkcs1v15Encrypt, message)
}

pub(crate) fn verify_password(password: &[u8], hash: &str) -> Result<(), argon2::password_hash::Error>
{
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hash)?;
    argon2.verify_password(password, &parsed_hash)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub jti: u32,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

pub(crate) fn generate_token(jti: u32, sub: &str, duration: u32, key: &[u8]) -> Result<String, jsonwebtoken::errors::Error>
{
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let iat = now.as_secs();
    let exp = iat + duration as u64;
    let claims = TokenClaims {
        jti,
        sub: sub.to_owned(),
        iat,
        exp
    };
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(key);
    let token = encode(&header, &claims, &encoding_key)?;
    Ok(token)
}

pub(crate) fn decode_token(token: &str, key: &[u8], exp_flag: bool) -> Result<TokenClaims, jsonwebtoken::errors::Error>
{
    let decoding_key = DecodingKey::from_secret(key);
    let mut validation = Validation::new(Algorithm::HS256);
    let req_claim: Vec<&str> = if exp_flag {
        ["exp"].to_vec()
    } else {
        [].to_vec()
    };
    validation.set_required_spec_claims(&req_claim);
    let token_data = decode::<TokenClaims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
