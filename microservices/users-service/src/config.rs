use std::env;

pub struct Config {
    pub microservice_url: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            microservice_url: env::var("USERS_SERVICE")
                .unwrap_or_else(|_| "127.0.0.1:50052".to_string()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/social-network".to_string()),
        }
    }
}