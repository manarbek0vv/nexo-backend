use regex::Regex;
use std::sync::LazyLock;

use crate::proto::auth::{SignInRequest, SignUpRequest};

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

pub fn check_password(password: &str) -> Result<(), &'static str> {
    let length = password.len();

    if !(6..=32).contains(&length) {
        return Err("Password must be 6-32 characters long");
    }

    if !password.chars().all(|c| c.is_ascii_graphic()) {
        return Err("Password must contain only Latin letters, digits and special characters");
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter");
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err("Password must contain at least one lowercase letter");
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one digit");
    }

    Ok(())
}

pub fn check_username(username: &str) -> Result<(), &'static str> {
    let length = username.len();

    if !(3..=32).contains(&length) {
        return Err("Username must be 3-32 characters long")
    }

    if !username.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("Username must contain only Latin characters and digits");
    }

    Ok(())
}

pub fn check_email(email: &str) -> Result<(), &'static str> {    
    if email.is_empty() {
        return Err("Email cannot be empty");
    }

    if email.len() > 254 {
        return Err("Email is too long");
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err("Invalid email format");
    }

    Ok(())
}

// ---------- Validating Specific Procedures ----------

pub fn validate_sign_up(input: SignUpRequest) -> Result<SignUpRequest, &'static str> {
    check_username(&input.username)?;
    check_email(&input.email)?;
    check_password(&input.password)?;

    Ok(input)
}

pub fn validate_sign_in(input: SignInRequest) -> Result<SignInRequest, &'static str> {
    check_email(&input.email)?;
    check_password(&input.password)?;

    Ok(input)
}