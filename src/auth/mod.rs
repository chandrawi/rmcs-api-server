pub mod api;
pub mod role;
pub mod user;
pub mod token;
pub mod auth;

// operation error message
const API_NOT_FOUND: &str = "requested api not found";
const API_CREATE_ERR: &str = "create api error";
const API_UPDATE_ERR: &str = "update api error";
const API_DELETE_ERR: &str = "delete api error";
const PROC_NOT_FOUND: &str = "requested procedure not found";
const PROC_CREATE_ERR: &str = "create procedure error";
const PROC_UPDATE_ERR: &str = "update procedure error";
const PROC_DELETE_ERR: &str = "delete procedure error";
const ROLE_NOT_FOUND: &str = "requested role not found";
const ROLE_CREATE_ERR: &str = "create role error";
const ROLE_UPDATE_ERR: &str = "update role error";
const ROLE_DELETE_ERR: &str = "delete role error";
const ADD_ACCESS_ERR: &str = "add role access error";
const RMV_ACCESS_ERR: &str = "remove role access error";
const USER_NOT_FOUND: &str = "requested user not found";
const USER_CREATE_ERR: &str = "create user error";
const USER_UPDATE_ERR: &str = "update user error";
const USER_DELETE_ERR: &str = "delete user error";
const ADD_ROLE_ERR: &str = "add user role error";
const RMV_ROLE_ERR: &str = "remove user role error";
const TOKEN_NOT_FOUND: &str = "requested token not found";
const TOKEN_CREATE_ERR: &str = "create token error";
const TOKEN_UPDATE_ERR: &str = "update token error";
const TOKEN_DELETE_ERR: &str = "delete token error";
const API_ID_NOT_FOUND: &str = "requested api id not found";
const USERNAME_NOT_FOUND: &str = "requested username not found";
const KEY_IMPORT_ERR: &str = "key import error";
const DECRYPT_ERR: &str = "decrypt password error";
const ENCRYPT_ERR: &str = "encrypt key error";
const PASSWORD_MISMATCH: &str = "password does not match";
const CREATE_TOKEN_ERR: &str = "error create token";
const UPDATE_TOKEN_ERR: &str = "error update token";
const DELETE_TOKEN_ERR: &str = "error delete token";
const GENERATE_TOKEN_ERR: &str = "error generate token";
const TOKEN_MISMATCH: &str = "refresh token is not match";
const TOKEN_UNVERIFIED: &str = "token unverified";
