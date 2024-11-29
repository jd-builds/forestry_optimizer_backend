mod application;
mod common;
mod domain;
mod infrastructure;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::application::{
    config::AppConfig,
    server,
    AppState,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging first
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();

    info!("Starting application");
    
    let config = AppConfig::load()?;
    info!("Config loaded successfully");
    
    let state = AppState::new(
        config.auth_service(),
        Some(config.org_service())
    );
    info!("Application state initialized");

    info!("Starting server on {}:{}", config.host, config.port);
    server::run(config, state).await
}
