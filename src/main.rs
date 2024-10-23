use dotenv::dotenv;
use log::{info, error};
use std::io;

mod schema;
mod config;
mod db;
mod routes;
mod server;
mod models;
mod repositories;
mod api;
mod error;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::load_config().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        io::Error::new(io::ErrorKind::Other, e)
    })?;

    let pool = db::create_connection_pool(&config.database_url).map_err(|e| {
        error!("Failed to create database pool: {}", e);
        io::Error::new(io::ErrorKind::Other, e)
    })?;

    info!("Starting server on {}:{}", config.host, config.port);
    
    server::run(config, pool).await
}
