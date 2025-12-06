use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tonic::transport::Server;
use crate::config::Config;
use crate::proto::users::users_server::{ UsersServer };

use crate::repository::UsersRepository;
use crate::service::UsersService;

pub mod service;
pub mod repository;
pub mod model;
pub mod error;
pub mod proto;
pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();

    let address: SocketAddr = config.microservice_url.parse()?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let repository = UsersRepository::new(db);
    let service = UsersService::new(repository);

    println!("Users service listening on {}", address);

    Server::builder()
        .add_service(UsersServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}