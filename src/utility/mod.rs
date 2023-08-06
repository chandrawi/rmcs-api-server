pub(crate) mod token;
pub mod config;
pub mod validator;

use rsa::{RsaPrivateKey, Pkcs1v15Encrypt, RsaPublicKey};
use pkcs8::{DecodePublicKey, EncodePublicKey};
use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use rand::{thread_rng, Rng};
use tonic::{Request, Status};

pub fn generate_transport_keys() -> Result<(RsaPrivateKey, RsaPublicKey), rsa::Error>
{
    let mut rng = thread_rng();
    let bits = 1024;
    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok((priv_key, pub_key))
}

pub fn export_public_key(pub_key: RsaPublicKey) -> Result<Vec<u8>, spki::Error>
{
    let pub_der = pub_key.to_public_key_der()?.to_vec();
    Ok(pub_der)
}

pub fn import_public_key(pub_der: &[u8]) -> Result<RsaPublicKey, spki::Error>
{
    let pub_key = RsaPublicKey::from_public_key_der(pub_der)?;
    Ok(pub_key)
}

pub fn decrypt_message(ciphertext: &[u8], priv_key: RsaPrivateKey) -> Result<Vec<u8>, rsa::Error>
{
    priv_key.decrypt(Pkcs1v15Encrypt, ciphertext)
}

pub fn encrypt_message(message: &[u8], pub_key: RsaPublicKey) -> Result<Vec<u8>, rsa::Error>
{
    pub_key.encrypt(&mut thread_rng(), Pkcs1v15Encrypt, message)
}

pub(crate) fn verify_password(password: &[u8], hash: &str) -> Result<(), argon2::password_hash::Error>
{
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hash)?;
    argon2.verify_password(password, &parsed_hash)
}

pub fn generate_access_key() -> Vec<u8>
{
    (0..32).map(|_| thread_rng().gen()).collect()
}

pub fn interceptor(mut request: Request<()>) -> Result<Request<()>, Status>
{
    let token = match request.metadata().get("authorization") {
        Some(value) => match value.to_str() {
            Ok(v) => v,
            Err(e) => return Err(Status::unauthenticated(format!("{}", e)))
        },
        None => return Err(Status::unauthenticated("Token not found"))
    };
    let token = match token.strip_prefix("Bearer ") {
        Some(value) => value.to_owned(),
        None => return Err(Status::unauthenticated("authorization header must in format 'Bearer <TOKEN>'"))
    };
    request.extensions_mut().insert(token);
    Ok(request)
}
