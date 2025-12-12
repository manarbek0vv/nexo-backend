use sqlx::{ Pool, Postgres };
use crate::model::{CreateUserRepo, GetUserRepo, User};
use crate::error::RepositoryError;
use crate::proto::users::{CreateUserRequest, GetUserRequest};

#[derive(Clone, Debug)]
pub struct UsersRepository {
    db: Pool<Postgres>
}

impl UsersRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn get_user(
        &self,
        value: GetUserRequest,
    ) -> Result<User, RepositoryError> {
        let GetUserRepo { email } = value.into();

        let result = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.db)
        .await?;

        Ok(result)
    }

    pub async fn create_user(
        &self,
        value: CreateUserRequest,
    ) -> Result<User, RepositoryError> {
        let CreateUserRepo { username, email, password } = value.into();

        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password
            "#,
            username,
            email,
            password,
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