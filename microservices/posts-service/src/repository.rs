use sqlx::{Pool, Postgres};

use crate::model::{CreatePostRepo, DeletePostRepo, GetPostRepo, Post, UpdatePostRepo};
use crate::proto::proto::posts::{CreatePostRequest, DeletePostRequest, GetPostRequest, UpdatePostRequest};
use crate::{error::RepositoryError};

#[derive(Debug, Clone)]
pub struct PostsRepository {
    pub db: Pool<Postgres>
}

impl PostsRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn get_post(
        &self,
        value: GetPostRequest,
    ) -> Result<Post, RepositoryError> {
        let GetPostRepo { id } = value.try_into()?;

        let result = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(result)
    }

    pub async fn get_posts(
        &self
    ) -> Result<Vec<Post>, RepositoryError> {
        let result = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(result)
    }

    pub async fn create_post(
        &self,
        value: CreatePostRequest,
    ) -> Result<Post, RepositoryError> {
        let CreatePostRepo { title, description, user_id } = value.try_into()?;

        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, description, user_id)
            VALUES ($1, $2, $3)
            RETURNING id, title, description, user_id, created_at, updated_at
            "#,
            title,
            description,
            user_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(post)
    }

    pub async fn update_post(
        &self,
        value: UpdatePostRequest,
    ) -> Result<Post, RepositoryError> {
        let UpdatePostRepo { id, title, description } = value.try_into()?;

        let result = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $1, description = $2
            WHERE id = $3
            RETURNING id, title, description, user_id, created_at, updated_at
            "#, title, description, id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(result)
    }

    pub async fn delete_post(
        &self,
        value: DeletePostRequest,
    ) -> Result<Post, RepositoryError> {
        let DeletePostRepo { id } = value.try_into()?;

        let result = sqlx::query_as!(
            Post,
            r#"
            DELETE FROM posts
            WHERE id = $1
            RETURNING id, title, description, user_id, created_at, updated_at
            "#, id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(result)
    }
}