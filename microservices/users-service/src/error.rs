use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user with this email not found")]
    UserNotFound,

    #[error("user already exists")]
    UserAlreadyExists
}

pub fn map_repo_err(err: RepositoryError) -> Status {
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