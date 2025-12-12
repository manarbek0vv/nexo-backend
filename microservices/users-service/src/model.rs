use tonic::Status;
use uuid::Uuid;

use crate::proto::users::{self, CreateUserRequest, GetUserRequest};

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<User> for users::User {
    type Error = Status;

    fn try_from(value: User) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.to_string(),
            username: value.username.to_string(),
            email: value.email.to_string(),
            password: value.password.to_string(),
        })
    }
}

// --------------------

pub struct GetUserRepo {
    pub email: String
}

impl From<GetUserRequest> for GetUserRepo {
    fn from(value: GetUserRequest) -> Self {
        Self {
            email: value.email
        }
    }
}

// --------------------

pub struct CreateUserRepo {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<CreateUserRequest> for CreateUserRepo {
    fn from(value: CreateUserRequest) -> Self {
        Self {
            email: value.email,
            password: value.password,
            username: value.username,
        }
    }
}