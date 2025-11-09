pub mod api;
pub mod role;
pub mod user;
pub mod profile;
pub mod token;
pub mod auth;

// operation error message
const TOKEN_NOT_FOUND: &str = "requested token not found";
const KEY_IMPORT_ERR: &str = "key import error";
const DECRYPT_ERR: &str = "decrypt password error";
const ENCRYPT_ERR: &str = "encrypt key error";
const PASSWORD_MISMATCH: &str = "password does not match";
const GENERATE_TOKEN_ERR: &str = "error generate token";
const TOKEN_MISMATCH: &str = "token is not match";
const TOKEN_UNVERIFIED: &str = "token unverified";
