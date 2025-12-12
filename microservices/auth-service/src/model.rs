use crate::proto::{auth::{SignUpRequest}, users::CreateUserRequest};

impl From<SignUpRequest> for CreateUserRequest {
    fn from(value: SignUpRequest) -> Self {
        Self {
            username: value.username,
            email: value.email,
            password: value.password,
        }
    }
}