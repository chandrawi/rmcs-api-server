use tonic::{Status, Extensions};
use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};
use super::token::TokenClaims;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AccessSchema {
    pub procedure: String,
    pub roles: Vec<String>
}

pub trait AccessValidator {

    fn with_validator(self, token_key: &[u8], accesses: &[AccessSchema]) -> Self;

    fn token_key(&self) -> Vec<u8>;

    fn accesses(&self) -> Vec<AccessSchema>;

    fn validate(&self, extension: &Extensions, procedure: &str) -> Result<(), Status>
    {
        // return ok if service doesn't configured to use validation
        if self.accesses().len() == 0 {
            return Ok(());
        }
        // decode token from request extension and get token claims
        let token = extension.get::<String>()
            .ok_or(Status::unauthenticated("Extension not found"))?;
        let decoding_key = DecodingKey::from_secret(self.token_key().as_slice());
        let validation = Validation::new(Algorithm::HS256);
        let claims = decode::<TokenClaims>(token, &decoding_key, &validation)
            .map_err(|_| Status::unauthenticated("Token is broken or expired"))?
            .claims;
        // check if the role in token claims has accsess rights to the procedure
        let access = self.accesses()
            .into_iter()
            .filter(|a| a.procedure == procedure)
            .next()
            .ok_or(Status::internal("Procedure access not found"))?;
        let role = access.roles
            .into_iter()
            .filter(|r| *r == claims.sub)
            .next();
        match role {
            Some(_) => Ok(()),
            None => Err(Status::unauthenticated(
                format!("{} doesn't have access rights to {} procedure", claims.sub, procedure)
            ))
        }
    }

}
