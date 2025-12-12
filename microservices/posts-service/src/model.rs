use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use crate::proto::proto::posts::{CreatePostRequest, DeletePostRequest, GetPostRequest, Post as ProtoPost, UpdatePostRequest};
use crate::{domain::time::datetime_to_timestamp};

#[derive(Debug, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Post> for ProtoPost {
    fn from(post: Post) -> ProtoPost {
        ProtoPost {
            id: post.id.to_string(),
            title: post.title,
            description: post.description,
            user_id: post.user_id.to_string(),
            created_at: datetime_to_timestamp(post.created_at),
            updated_at: datetime_to_timestamp(post.updated_at),
        }
    }
}

// ----------------------------

pub struct GetPostRepo {
    pub id: Uuid,
}

impl TryFrom<GetPostRequest> for GetPostRepo {
    type Error = uuid::Error;

    fn try_from(value: GetPostRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?
        })
    }
}

// ----------------------------

pub struct CreatePostRepo {
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
}

impl TryFrom<CreatePostRequest> for CreatePostRepo {
    type Error = uuid::Error;

    fn try_from(value: CreatePostRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title,
            description: value.description,
            user_id: Uuid::parse_str(&value.user_id)?,
        })
    }
}

// ------------------------------

pub struct UpdatePostRepo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

impl TryFrom<UpdatePostRequest> for UpdatePostRepo {
    type Error = uuid::Error;

    fn try_from(value: UpdatePostRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            title: value.title,
            description: value.description,
        })
    }
}

// -----------------------------

pub struct DeletePostRepo {
    pub id: Uuid
}

impl TryFrom<DeletePostRequest> for DeletePostRepo {
    type Error = uuid::Error;

    fn try_from(value: DeletePostRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
        })
    }
}