use thiserror::Error;

use crate::model::CommentConvertError;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Comment with this id not found")]
    CommentNotFound,

    #[error("Invalid UUID")]
    InternalServerError(#[from] CommentConvertError),
}