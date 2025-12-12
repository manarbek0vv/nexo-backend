use sqlx::{Pool, Postgres};

use crate::{error::RepositoryError, model::{AddCommentRepo, Comment, DeleteCommentRepo, GetCommentRepo, GetCommentsRepo, UpdateCommentRepo}, proto::proto::comments::{AddCommentRequest, DeleteCommentRequest, GetCommentRequest, GetCommentsRequest, UpdateCommentRequest}};

#[derive(Debug, Clone)]
pub struct CommentsRepository {
    pub db: Pool<Postgres>
}

impl CommentsRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn get_comment(
        &self,
        value: &GetCommentRequest
    ) -> Result<Comment, RepositoryError> {
        let GetCommentRepo { id } = value.try_into()?;

        let comment = sqlx::query_as!(
            Comment,
            r#"
            SELECT id, content, post_id, user_id, created_at, updated_at
            FROM comments
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(comment)
    }

    pub async fn get_comments(
        &self,
        value: &GetCommentsRequest
    ) -> Result<Vec<Comment>, RepositoryError> {
        let GetCommentsRepo { post_id } = value.try_into()?;

        let comment = sqlx::query_as!(
            Comment,
            r#"
            SELECT id, content, post_id, user_id, created_at, updated_at
            FROM comments
            WHERE post_id = $1
            "#,
            post_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(comment)
    }

    pub async fn add_comment(
        &self,
        value: &AddCommentRequest,
    ) -> Result<Comment, RepositoryError> {
        let AddCommentRepo {content, post_id, user_id} = value.try_into()?;

        let comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (content, user_id, post_id)
            VALUES ($1, $2, $3)
            RETURNING id, content, user_id, post_id, created_at, updated_at
            "#,
            content,
            user_id,
            post_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(comment)
    }

    pub async fn update_comment(
        &self,
        value: &UpdateCommentRequest,
    ) -> Result<Comment, RepositoryError> {
        let UpdateCommentRepo { id, content, ..  } = value.try_into()?;

        let comment = sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET content = $1
            WHERE id = $2
            RETURNING id, content, user_id, post_id, created_at, updated_at
            "#,
            content,
            id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(comment)
    }

    pub async fn delete_comment(
        &self,
        value: &DeleteCommentRequest,
    ) -> Result<Comment, RepositoryError> {
        let DeleteCommentRepo { id } = value.try_into()?;

        let comment = sqlx::query_as!(
            Comment,
            r#"
            DELETE FROM comments
            WHERE id = $1
            RETURNING id, content, user_id, post_id, created_at, updated_at
            "#,
            id,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(comment)
    }
}