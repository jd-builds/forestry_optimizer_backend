use config::Config;
use tracing::info;

mod api;
mod config;
mod db;
mod docs;
mod errors;
mod middleware;
mod server;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load()?;

    info!("Starting application in {} mode", config.environment);

    server::run(config).await
}
