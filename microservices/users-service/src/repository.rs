use sqlx::{ Pool, Postgres };
use crate::model::{NewUser, User};
use crate::error::RepositoryError;

#[derive(Clone, Debug)]
pub struct UsersRepository {
    db: Pool<Postgres>
}

impl UsersRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<User, RepositoryError> {
        let result = sqlx::query_as!(
            User,
        r#"
            SELECT id, username, email, password
            FROM users
            WHERE email = $1
        "#, email)
        .fetch_optional(&self.db)
        .await?;

        result.ok_or(RepositoryError::UserNotFound)
    }

    pub async fn create_user(
        &self,
        data: &NewUser,
    ) -> Result<User, RepositoryError> {
        let result = sqlx::query_as!(
            User,
        r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password
        "#,
        data.username,
        data.email,
        data.password,
        )
        .fetch_one(&self.db)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(err) => {
                if let Some(db_err) = err.as_database_error() {
                    if db_err.is_unique_violation() {
                            return Err(RepositoryError::UserAlreadyExists)
                        }
                }

                Err(RepositoryError::DatabaseError(err))
            }
        }
    }
}