//! HTTP server configuration and setup
//! 
//! This module handles the setup and configuration of the HTTP server,
//! including middleware, routes, and error handlers.

use crate::{
    application::{
        api::routes,
        AppState,
        config::AppConfig,
    },
    common::error::ApiError,
};
use actix_web::{
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpServer,
};
use std::sync::Arc;
use tracing::info;

pub async fn run(config: AppConfig, state: AppState) -> std::io::Result<()> {
    info!("Starting HTTP server on {}:{}", config.host, config.port);

    let state = web::Data::new(Arc::new(state));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                ApiError::validation(
                    "Invalid JSON payload",
                    Some(serde_json::json!({ "error": err.to_string() })),
                ).into()
            }))
            .service(
                web::scope("/api")
                    .wrap(Logger::default())
                    .wrap(NormalizePath::new(TrailingSlash::Trim))
                    .configure(routes::configure)
            )
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
} 