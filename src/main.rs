use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};

mod api;
mod config;
mod db;
mod error;
mod models;
mod repositories;
mod routes;
mod schema;
mod server;

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
