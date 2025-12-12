use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use sqlx::types::{Uuid, uuid};
use crate::domain::time::{datetime_to_timestamp};
use crate::proto::proto::comments::{AddCommentRequest, Comment as ProtoComment, DeleteCommentRequest, GetCommentRequest, GetCommentsRequest, UpdateCommentRequest};

#[derive(Debug, thiserror::Error)]
pub enum CommentConvertError {
    #[error("invalid uuid: {0}")]
    InvalidUuid(#[from] uuid::Error),
}

#[derive(Debug, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Comment> for ProtoComment {
    fn from(value: Comment) -> ProtoComment {
        ProtoComment {
            id: value.id.to_string(),
            content: value.content,
            user_id: value.user_id.to_string(),
            post_id: value.post_id.to_string(),
            created_at: Some(datetime_to_timestamp(value.created_at)),
            updated_at: Some(datetime_to_timestamp(value.updated_at)),
        }
    }
}
// -----------------------------

pub struct GetCommentRepo {
    pub id: Uuid
}

impl TryFrom<&GetCommentRequest> for GetCommentRepo {
    type Error = CommentConvertError;

    fn try_from(value: &GetCommentRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?
        })
    }
}

// -----------------------------

pub struct  GetCommentsRepo {
    pub post_id: Uuid
}

impl TryFrom<&GetCommentsRequest> for GetCommentsRepo {
    type Error = CommentConvertError;

    fn try_from(value: &GetCommentsRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            post_id: Uuid::parse_str(&value.post_id)?
        })
    }
}

// -----------------------------

pub struct AddCommentRepo {
    pub content: String,
    pub post_id: Uuid,
    pub user_id: Uuid,
}

impl TryFrom<&AddCommentRequest> for AddCommentRepo {
    type Error = CommentConvertError;

    fn try_from(value: &AddCommentRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            content: value.content.to_string(),
            post_id: Uuid::parse_str(&value.post_id)?,
            user_id: Uuid::parse_str(&value.user_id)?,
        })
    }
}

// ---------------------------

pub struct UpdateCommentRepo {
    pub id: Uuid,
    pub content: String,
    pub post_id: Uuid,
    pub user_id: Uuid,
}

impl TryFrom<&UpdateCommentRequest> for UpdateCommentRepo {
    type Error = CommentConvertError;

    fn try_from(value: &UpdateCommentRequest) -> Result<Self, Self::Error> {
        Ok(UpdateCommentRepo {
            id: Uuid::parse_str(&value.id)?,
            content: value.content.to_string(),
            post_id: Uuid::parse_str(&value.post_id)?,
            user_id: Uuid::parse_str(&value.user_id)?,
        })
    }
}

// ---------------------------


pub struct DeleteCommentRepo {
    pub id: Uuid,
}

impl TryFrom<&DeleteCommentRequest> for DeleteCommentRepo {
    type Error = CommentConvertError;

    fn try_from(value: &DeleteCommentRequest) -> Result<Self, Self::Error> {
        Ok(DeleteCommentRepo {
            id: Uuid::parse_str(&value.id)?,
        })
    }
}