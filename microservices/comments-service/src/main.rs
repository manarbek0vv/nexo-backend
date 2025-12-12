use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use crate::{config::Config, proto::proto::comments::comments_server::CommentsServer, repository::CommentsRepository, service::CommentsService};

pub mod service;
pub mod proto;
pub mod config;
pub mod repository;
pub mod model;
pub mod domain;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();

    let addr: SocketAddr = config.microservice_url.parse()?;

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let repository = CommentsRepository::new(db);
    let service = CommentsService::new(repository);

    println!("Comments service listening on {}", addr);

    Server::builder()
        .add_service(CommentsServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}