use tonic::{Status, Extensions};
use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};
use async_trait::async_trait;
use super::token::TokenClaims;
use rmcs_auth_db::Auth;

const EXT_NOT_FOUND: &str = "Extension not found";
const TOKEN_EXPIRED: &str = "Token is broken or expired";
const PROC_NOT_FOUND: &str = "Procedure access not found";
const USER_UNREGISTERED: &str = "user has not registered";
const ACCESS_RIGHT_ERR: &str = "doesn't has access rights";

#[derive(Debug, Clone)]
pub struct AccessSchema {
    pub procedure: String,
    pub roles: Vec<String>
}

pub trait AccessValidator {

    fn with_validator(self, token_key: &[u8], accesses: &[AccessSchema]) -> Self;

    fn token_key(&self) -> Vec<u8>;

    fn accesses(&self) -> Vec<AccessSchema>;

    fn construct_accesses(accesses: &[AccessSchema], procedures: &[&str]) -> Vec<AccessSchema>
    {
        procedures.into_iter().map(|&s| AccessSchema {
            procedure: s.to_owned(),
            roles: accesses.iter()
                .filter(|&a| a.procedure == s)
                .map(|a| a.roles.clone())
                .next()
                .unwrap_or_default()
        })
        .collect()
    }

    fn validate(&self, extension: &Extensions, procedure: &str) -> Result<(), Status>
    {
        // return ok if service doesn't configured to use validation
        if self.accesses().len() == 0 {
            return Ok(());
        }
        // decode token from request extension and get token claims
        let token = extension.get::<String>()
            .ok_or(Status::unauthenticated(EXT_NOT_FOUND))?;
        let decoding_key = DecodingKey::from_secret(self.token_key().as_slice());
        let validation = Validation::new(Algorithm::HS256);
        let claims = decode::<TokenClaims>(token, &decoding_key, &validation)
            .map_err(|_| Status::unauthenticated(TOKEN_EXPIRED))?
            .claims;
        // check if the role in token claims has accsess rights to the procedure
        let access = self.accesses()
            .into_iter()
            .filter(|a| a.procedure == procedure)
            .next()
            .ok_or(Status::internal(PROC_NOT_FOUND))?;
        let role = access.roles
            .into_iter()
            .filter(|r| *r == claims.sub)
            .next();
        match role {
            Some(_) => Ok(()),
            None => Err(Status::unauthenticated(
                format!("Role {} {}", claims.sub, ACCESS_RIGHT_ERR)
            ))
        }
    }

}

pub enum ValidatorKind {
    User(u32),
    Root
}

#[async_trait]
pub trait AuthValidator {

    fn with_validator(self) -> Self;

    fn validator_flag(&self) -> bool;

    fn auth_db(&self) -> &Auth;

    async fn validate(&self, extension: &Extensions, kind: ValidatorKind) -> Result<(), Status>
    {
        // return ok if service doesn't configured to use validation
        if !self.validator_flag() {
            return Ok(());
        }
        // get user id from extension
        let token = extension.get::<String>()
            .ok_or(Status::unauthenticated(EXT_NOT_FOUND))?;
        let result = self.auth_db().list_auth_token(token).await;
        let user_id = match result {
            Ok(value) => match value.into_iter().next() {
                Some(v) => v.user_id,
                None => return Err(Status::unauthenticated(USER_UNREGISTERED))
            },
            Err(_) => return Err(Status::unauthenticated(USER_UNREGISTERED))
        };
        // check input user id or root user (user_id = 0)
        if let ValidatorKind::User(id) = kind {
            if id == user_id {
                return Ok(());
            }
        }
        if user_id == 0 {
            Ok(())
        } else {
            Err(Status::unauthenticated(format!("User {} {}", user_id, ACCESS_RIGHT_ERR)))
        }
    }

}
