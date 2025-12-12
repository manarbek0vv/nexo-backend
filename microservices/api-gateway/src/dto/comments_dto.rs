use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tonic::Status;
use uuid::Uuid;
use crate::{domain::time::timestamp_to_datetime, proto::comments::{self, Comment as ProtoComment}};


#[derive(Debug, Serialize)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<ProtoComment> for Comment {
    type Error = Status;

    fn try_from(value: ProtoComment) -> Result<Self, Self::Error> {
        Ok(Comment {
            id: Uuid::parse_str(&value.id)
                .map_err(|_| Status::internal("Error converting UUID"))?,
            content: value.content,
            user_id: Uuid::parse_str(&value.user_id)
                .map_err(|_| Status::internal("Error converting UUID"))?,
            post_id: Uuid::parse_str(&value.post_id)
                .map_err(|_| Status::internal("Error converting UUID"))?,
            created_at: timestamp_to_datetime(value.created_at),
            updated_at: timestamp_to_datetime(value.updated_at),
        })
    }
}

// ---------- Get Post ----------
#[derive(Deserialize)]
pub struct GetCommentRequest {
    pub id: Uuid,
}

impl From<GetCommentRequest> for comments::GetCommentRequest {
    fn from(value: GetCommentRequest) -> Self {
        Self {
            id: value.id.to_string()
        }
    }
}

#[derive(Serialize)]
pub struct GetCommentResponse {
    pub comment: Comment
}

// ---------- Get Comments ----------
#[derive(Deserialize)]
pub struct GetCommentsRequest {
    pub post_id: Uuid
}

impl From<GetCommentsRequest> for comments::GetCommentsRequest {
    fn from(value: GetCommentsRequest) -> Self {
        Self {
            post_id: value.post_id.to_string()
        }
    }
}

#[derive(Serialize)]
pub struct GetCommentsResponse {
    pub comments: Vec<Comment>
}

// ---------- Create Post ----------
#[derive(Deserialize)]
pub struct AddCommentRequest {
    pub content: String,
    pub user_id: Uuid,
    pub post_id: Uuid,
}

impl From<AddCommentRequest> for comments::AddCommentRequest {
    fn from(value: AddCommentRequest) -> Self {
        Self {
            content: value.content,
            user_id: value.user_id.to_string(),
            post_id: value.post_id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct AddCommentResponse {
    pub comment: Comment
}

// ---------- Update Post ----------
#[derive(Deserialize)]
pub struct UpdateCommentRequest {
    pub id: Uuid,
    pub content: String,
    pub user_id: Uuid,
    pub post_id: Uuid,
}

impl From<UpdateCommentRequest> for comments::UpdateCommentRequest {
    fn from(value: UpdateCommentRequest) -> Self {
        Self {
            id: value.id.to_string(),
            content: value.content,
            user_id: value.user_id.to_string(),
            post_id: value.post_id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct UpdateCommentResponse {
    pub comment: Comment
}

// ---------- Delete Post ----------
#[derive(Deserialize)]
pub struct DeleteCommentRequest {
    pub id: Uuid,
}

impl From<DeleteCommentRequest> for comments::DeleteCommentRequest {
    fn from(value: DeleteCommentRequest) -> Self {
        Self {
            id: value.id.to_string()
        }
    }
}

#[derive(Serialize)]
pub struct DeleteCommentResponse {
    pub comment: Comment
}