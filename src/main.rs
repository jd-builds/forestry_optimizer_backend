use config::Config;
use dotenv::dotenv;
use log::{info, warn};

mod api;
mod config;
mod db;
mod docs;
mod errors;
mod middleware;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file and check if it was found
    if dotenv().is_err() {
        // Log warning but don't fail - env vars might be set directly
        warn!("No .env file found - using environment variables");
    }

    let config = Config::load()?;
    info!("Starting application in {} mode", config.environment);

    server::run(config).await
}
