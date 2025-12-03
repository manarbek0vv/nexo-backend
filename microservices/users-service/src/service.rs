use tonic::{ Request, Response, Status };
use crate::repository::{RepositoryError, UsersRepository};
use crate::structs::NewUser;
use crate::users::users_server::Users;
use crate::users::{CreateUserRequest, GetUserRequest, User};

#[derive(Debug, Clone)]
pub struct UsersService {
    repository: UsersRepository
}

impl UsersService {
    pub fn new(repository: UsersRepository) -> Self {
        Self { repository }
    }
}

fn map_repo_err(err: RepositoryError) -> Status {
    match err {
        RepositoryError::UserNotFound => {
            Status::not_found("user with email not found")
        },
        RepositoryError::UserAlreadyExists => {
            Status::already_exists("user with this email already exists")
        },
        RepositoryError::DatabaseError(_) => {
            Status::internal("internal server error")
        }
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
            .get_user_by_email(&request.email)
            .await
            .map_err(map_repo_err)?;

        let response = User {
            id: user.id,
            email: user.email,
            username: user.username,
            password: user.password,
        };

        Ok(Response::new(response))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let request = request.into_inner();

        let request_for_repo = NewUser {
            username: request.username,
            email: request.email,
            password: request.password,
        };

        let user = self
        .repository
        .create_user(&request_for_repo)
        .await
        .map_err(map_repo_err)?;

        let response = User {
                id: user.id,
                username: user.username,
                email: user.email,
                password: user.password,
        };

        Ok(Response::new(response))
    }
}