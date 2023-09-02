use tonic::{Status, Request, service::Interceptor, metadata::MetadataValue};

#[derive(Debug, Clone)]
pub struct TokenInterceptor(pub String);

impl Interceptor for TokenInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request.metadata_mut().insert(
            "authorization", 
            MetadataValue::try_from(String::from("Bearer ") + &self.0).unwrap()
        );
        Ok(request)
    }
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
