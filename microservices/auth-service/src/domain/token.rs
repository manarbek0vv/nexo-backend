use std::env;

use serde::{ Serialize, Deserialize };
use jsonwebtoken::{ DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error };
use chrono::{ Duration, Utc };

#[derive(Clone)]
pub struct Payload {
    pub sub: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub username: String,
    pub exp: i64,
}

pub fn generate_access_token(claims: Payload) -> Result<String, Error> {
    let access_secret: String = env::var("JWT_ACCESS_TOKEN_SECRET")
        .unwrap_or_else(|_| "H7GF8FGG6D".to_string())
        .to_string();

    let expiration = (
        Utc::now() + Duration::minutes(15)
    ).timestamp();

    let token_claims = TokenClaims {
        sub: claims.sub,
        username: claims.username,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(access_secret.as_ref()),
    )
}

pub fn generate_refresh_token(claims: Payload) -> Result<String, Error> {
    let refresh_secret = env::var("JWT_REFRESH_TOKEN_SECRET")
        .unwrap_or_else(|_| "FGJD86DSGF".to_string())
        .to_string();

    let expiration = (
        Utc::now() + Duration::days(30)
    ).timestamp();

    let token_claims = TokenClaims {
        sub: claims.sub,
        username: claims.username,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(refresh_secret.as_ref()),
    )
}

pub fn verify_access_token(token: String) -> Result<TokenClaims, Error> {
    let access_secret: String = env::var("JWT_ACCESS_TOKEN_SECRET")
        .unwrap_or_else(|_| "H7GF8FGG6D".to_string())
        .to_string();

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(access_secret.as_ref()),
        &Validation::default(),
    )
    .map(|token| token.claims)
}

pub fn verify_refresh_token(token: String) -> Result<TokenClaims, Error> {
    let refresh_secret = env::var("JWT_REFRESH_TOKEN_SECRET")
        .unwrap_or_else(|_| "FGJD86DSGF".to_string())
        .to_string();

    decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(refresh_secret.as_ref()),
        &Validation::default(),
    )
    .map(|token| token.claims)
}

pub fn generate_tokens(claims: Payload) -> Result<(String, String), Box<dyn std::error::Error>> {
    let access_token = generate_access_token(claims.clone())?;
    let refresh_token = generate_refresh_token(claims.clone())?;

    Ok((access_token, refresh_token))
}