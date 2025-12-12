use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("user service internal error")]
    UserServiceInternal,

    // #[error("Password hashing error")]
    // PasswordHashingError,
}