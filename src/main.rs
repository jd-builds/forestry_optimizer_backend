use dotenv::dotenv;
use log::{info, error};
use std::io;
use env_logger::Env;

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
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting application");

    let config = config::load_config().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    let pool = db::create_connection_pool(&config.database_url).map_err(|e| {
        error!("Failed to create database pool: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    info!("Database pool created successfully");
    info!("Starting server on {}:{}", config.host, config.port);
    
    server::run(config, pool).await
}
