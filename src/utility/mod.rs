pub(crate) mod token;
pub mod config;
pub mod validator;
pub mod interceptor;
pub mod auth;
pub mod test;

use sha2::Sha256;
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep};
use pkcs8::{DecodePublicKey, EncodePublicKey};
use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};
use rand::thread_rng;

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
    let padding = Oaep::new_with_mgf_hash::<Sha256, Sha256>();
    priv_key.decrypt(padding, ciphertext)
}

pub fn encrypt_message(message: &[u8], pub_key: RsaPublicKey) -> Result<Vec<u8>, rsa::Error>
{
    let padding = Oaep::new_with_mgf_hash::<Sha256, Sha256>();
    pub_key.encrypt(&mut thread_rng(), padding, message)
}

pub(crate) fn verify_password(password: &[u8], hash: &str) -> Result<(), argon2::password_hash::Error>
{
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hash)?;
    argon2.verify_password(password, &parsed_hash)
}

pub(crate) fn handle_error(e: sqlx::Error) -> tonic::Status {
    return match e {
        sqlx::Error::RowNotFound => tonic::Status::not_found(e.to_string()),
        sqlx::Error::InvalidArgument(message) => tonic::Status::invalid_argument(message),
        sqlx::Error::Database(db_err) => {
            match db_err.try_downcast::<sqlx::postgres::PgDatabaseError>() {
                Ok(pg_err) => {
                    let message = match pg_err.detail() {
                        Some(detail) => format!("{} {}", pg_err.message(), detail),
                        None => pg_err.message().to_string()
                    };
                    if pg_err.code() == "23505" {
                        tonic::Status::already_exists(message)
                    } else if pg_err.code() == "23501" || pg_err.code() == "23502" || pg_err.code() == "23503" {
                        tonic::Status::invalid_argument(message)
                    } else {
                        tonic::Status::internal(message)
                    }
                },
                Err(e) => tonic::Status::internal(e.message())
            }
        },
        _ => tonic::Status::unknown(e.to_string())
    }
}
