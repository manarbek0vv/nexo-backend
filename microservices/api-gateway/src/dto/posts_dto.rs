use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tonic::Status;
use uuid::Uuid;

use crate::{domain::time::timestamp_to_datetime, proto::posts::{self, Post as ProtoPost}};

#[derive(Serialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<ProtoPost> for Post {
    type Error = Status;

    fn try_from(value: ProtoPost) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)
                .map_err(|_| Status::internal("Error converting UUID"))?,
            title: value.title,
            description: value.description,
            user_id: Uuid::parse_str(&value.user_id)
                .map_err(|_| Status::internal("Error converting UUID"))?,
            created_at: timestamp_to_datetime(value.created_at),
        })
    }
}

// ---------- Get Post ----------
#[derive(Deserialize)]
pub struct GetPostRequest {
    pub id: Uuid,
}

impl From<GetPostRequest> for posts::GetPostRequest {
    fn from(value: GetPostRequest) -> Self {
        Self {
            id: value.id.to_string()
        }
    }
}

#[derive(Serialize)]
pub struct GetPostResponse {
    pub post: Post
}

// ---------- Get Posts ----------
#[derive(Deserialize)]
pub struct GetPostsRequest { }

#[derive(Serialize)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>
}

// ---------- Create Post ----------
#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
}

impl From<CreatePostRequest> for posts::CreatePostRequest {
    fn from(value: CreatePostRequest) -> Self {
        Self {
            user_id: value.user_id.to_string(),
            title: value.title,
            description: value.description,
        }
    }
}

#[derive(Serialize)]
pub struct CreatePostResponse {
    pub post: Post
}

// ---------- Update Post ----------
#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

impl From<UpdatePostRequest> for posts::UpdatePostRequest {
    fn from(value: UpdatePostRequest) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            description: value.description,
        }
    }
}

#[derive(Serialize)]
pub struct UpdatePostResponse {
    pub post: Post
}

// ---------- Delete Post ----------
#[derive(Deserialize)]
pub struct DeletePostRequest {
    pub id: Uuid,
}

impl From<DeletePostRequest> for posts::DeletePostRequest {
    fn from(value: DeletePostRequest) -> Self {
        Self {
            id: value.id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct DeletePostResponse {
    pub post: Post
}