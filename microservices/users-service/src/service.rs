use tonic::{ Request, Response, Status };
use crate::error::map_repo_err;
use crate::repository::{UsersRepository};
use crate::proto::users::users_server::Users;
use crate::proto::users::{CreateUserRequest, GetUserRequest, User};

#[derive(Debug, Clone)]
pub struct UsersService {
    repository: UsersRepository
}

impl UsersService {
    pub fn new(repository: UsersRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        let user = self
            .repository
            .get_user(request)
            .await
            .map_err(map_repo_err)?;

        let response: User = user.try_into()
            .map_err(|_| Status::internal("Internal server error"))?;

        Ok(Response::new(response))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        let user = self
        .repository
        .create_user(request)
        .await
        .map_err(map_repo_err)?;

        let response: User = user.try_into()
            .map_err(|_| Status::internal("Internal server error"))?;

        Ok(Response::new(response))
    }
}