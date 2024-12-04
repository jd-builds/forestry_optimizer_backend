use optimizer::{Config, run};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load()?;

    // Initialize logging with environment-aware default level
    let default_log_level = match config.environment {
        optimizer::config::environment::Environment::Development |
        optimizer::config::environment::Environment::Staging => "debug",
        optimizer::config::environment::Environment::Production => "info",
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| default_log_level.into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting {} v{} in {} mode", 
        optimizer::NAME,
        optimizer::VERSION,
        config.environment
    );

    run().await
}
