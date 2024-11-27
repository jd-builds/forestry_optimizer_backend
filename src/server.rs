use crate::api::routes;
use crate::config::Config;
use crate::docs::openapi::ApiDoc;
use crate::middleware::{RateLimit, RequestId, SecurityHeaders};
use actix_web::{web, App, HttpServer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
};

pub async fn run(config: Config) -> std::io::Result<()> {
    setup_tracing(&config);

    let pool = web::Data::new(config.pool().clone());
    let host = config.host.clone();
    let port = config.port;

    info!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(RateLimit::new(100, 10))
            .wrap(RequestId)
            .wrap(SecurityHeaders::new())
            .app_data(pool.clone())
            .service(routes::v1_routes())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
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
