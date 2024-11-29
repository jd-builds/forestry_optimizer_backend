use crate::application::config::AppConfig;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::MakeWriter,
    layer::SubscriberExt,
    EnvFilter,
    Registry,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

/// Initialize the logging system
pub fn init_logging(
    config: &AppConfig,
) -> std::io::Result<()> {
    // Initialize log conversion
    LogTracer::init().expect("Failed to initialize log tracer");

    // Create file appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "application.log",
    );

    // Create subscriber with multiple layers
    let subscriber = get_subscriber(
        &config.environment.to_string(),
        "info",
        std::io::stdout,
        file_appender,
    );

    // Set as global default
    set_global_default(subscriber).expect("Failed to set subscriber");

    Ok(())
}

/// Get a configured subscriber
pub fn get_subscriber<Sink1, Sink2>(
    env: &str,
    env_filter: &str,
    sink1: Sink1,
    sink2: Sink2,
) -> impl Subscriber + Send + Sync
where
    Sink1: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    Sink2: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Create env filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Create formatting layers
    let formatting_layer1 = BunyanFormattingLayer::new(
        env.into(),
        sink1,
    );
    
    let formatting_layer2 = BunyanFormattingLayer::new(
        format!("{}_file", env),
        sink2,
    );

    // Compose layers
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer1)
        .with(formatting_layer2)
} 