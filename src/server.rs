use crate::api::routes;
use crate::config::Config;
use crate::docs::openapi::ApiDoc;
use crate::middleware::{RateLimit, RequestId, SecurityHeaders};
use crate::middleware::auth::Auth;
use actix_web::{web, App, HttpServer};
use std::time::Duration;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{info, warn};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
};

const GRACEFUL_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn run(config: Config) -> std::io::Result<()> {
    setup_tracing(&config);

    let pool = web::Data::new(config.pool().clone());
    let host = config.host.clone();
    let port = config.port;

    info!("Starting server on {}:{}", host, port);

    // Channel for shutdown coordination
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
    let shutdown_tx_clone = shutdown_tx.clone();

    // Spawn signal handler
    tokio::spawn(async move {
        handle_shutdown_signals(shutdown_tx_clone).await;
    });

    // Start server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(RateLimit::new(100, 10))
            .wrap(RequestId)
            .wrap(SecurityHeaders::new())
            .app_data(pool.clone())
            .service(
                web::scope("/v1")
                    .service(routes::v1::auth::routes())  // Public auth routes (login, register)
                    .service(
                        web::scope("")
                            .wrap(Auth::new())  // Protected routes
                            .service(routes::v1_routes())  // All other protected routes
                    )
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .workers(num_cpus::get())
    .shutdown_timeout(GRACEFUL_SHUTDOWN_TIMEOUT.as_secs())
    .run();

    // Store server handle for shutdown
    let server_handle = server.handle();
    
    // Spawn server task
    let server_task = tokio::spawn(server);

    // Wait for shutdown signal
    shutdown_rx.recv().await;
    info!("Initiating graceful shutdown");

    // Start graceful shutdown
    server_handle.stop(true).await;
    info!("Waiting for connections to drain");

    // Wait for ongoing requests to complete with timeout
    tokio::select! {
        _ = server_task => {
            info!("Server shutdown completed gracefully");
        }
        _ = sleep(GRACEFUL_SHUTDOWN_TIMEOUT) => {
            warn!("Server shutdown timed out after {:?}", GRACEFUL_SHUTDOWN_TIMEOUT);
        }
    }

    Ok(())
}

/// Handles OS signals for graceful shutdown
async fn handle_shutdown_signals(shutdown_tx: mpsc::Sender<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        }
        _ = terminate => {
            info!("Received terminate signal");
        }
    }

    info!("Sending shutdown signal");
    let _ = shutdown_tx.send(()).await;
}

fn setup_tracing(config: &Config) {
    let env_filter = if config.environment.is_development() {
        "debug"
    } else {
        "info"
    };

    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(config.environment.is_development())
        .compact()
        .with_source_location(false)
        .with_thread_names(false);

    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::new(env_filter))
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE)
        .event_format(format)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing");
}