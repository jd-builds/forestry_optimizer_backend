use rust_server::{Config, server::run};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load()?;

    // Initialize logging with environment-aware default level
    let default_log_level = match config.environment {
        rust_server::utils::environment::Environment::Development |
        rust_server::utils::environment::Environment::Staging => "debug",
        rust_server::utils::environment::Environment::Production => "info",
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| default_log_level.into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting {} v{} in {} mode", 
        rust_server::NAME,
        rust_server::VERSION,
        config.environment
    );

    run().await
}
