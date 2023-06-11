use tonic::{Request, Response, Status};
use rmcs_auth_db::Auth;
use rmcs_auth_api::user::user_service_server::UserService;
use rmcs_auth_api::user::{
    UserSchema, UserId, UserName, RoleId, UserUpdate,
    UserReadResponse, UserListResponse, UserCreateResponse, UserChangeResponse,
    ResponseStatus
};

pub struct UserServer {
    pub auth_db: Auth
}

impl UserServer {
    pub fn new(auth_db: Auth) -> Self {
        UserServer {
            auth_db
        }
    }
}

#[tonic::async_trait]
impl UserService for UserServer {

    async fn read_user(&self, request: Request<UserId>)
        -> Result<Response<UserReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_user(request.id).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(UserReadResponse { result, status }))
    }

    async fn read_user_by_name(&self, request: Request<UserName>)
        -> Result<Response<UserReadResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.read_user_by_name(&request.name).await;
        let (result, status) = match result {
            Ok(value) => (Some(value.into()), ResponseStatus::Success.into()),
            Err(_) => (None, ResponseStatus::Failed.into())
        };
        Ok(Response::new(UserReadResponse { result, status }))
    }

    async fn list_user_by_role(&self, request: Request<RoleId>)
        -> Result<Response<UserListResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.list_user_by_role(request.id).await;
        let (result, status) = match result {
            Ok(value) => (
                value.into_iter().map(|e| e.into()).collect(),
                ResponseStatus::Success.into()
            ),
            Err(_) => (Vec::new(), ResponseStatus::Failed.into())
        };
        Ok(Response::new(UserListResponse { result, status }))
    }

    async fn create_user(&self, request: Request<UserSchema>)
        -> Result<Response<UserCreateResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.create_user(
            &request.name,
            &request.email,
            &request.phone,
            &request.password
        ).await;
        let (id, status) = match result {
            Ok(value) => (value, ResponseStatus::Success.into()),
            Err(_) => (0, ResponseStatus::Failed.into())
        };
        Ok(Response::new(UserCreateResponse { id, status }))
    }

    async fn update_user(&self, request: Request<UserUpdate>)
        -> Result<Response<UserChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.update_user(
            request.id,
            request.name.as_deref(),
            request.email.as_deref(),
            request.phone.as_deref(),
            request.password.as_deref(),
            if request.update_key { Some(()) } else { None }
        ).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(UserChangeResponse { status }))
    }

    async fn delete_user(&self, request: Request<UserId>)
        -> Result<Response<UserChangeResponse>, Status>
    {
        let request = request.into_inner();
        let result = self.auth_db.delete_user(request.id).await;
        let status = match result {
            Ok(_) => ResponseStatus::Success.into(),
            Err(_) => ResponseStatus::Failed.into()
        };
        Ok(Response::new(UserChangeResponse { status }))
    }

}
