//! Server configuration and startup
//! 
//! This module handles the HTTP server setup, including middleware
//! configuration and route registration.

use crate::{
    api::{middleware::{RequestId, SecurityHeaders}, routes},
    utils::Config,
};
use actix_web::{
    middleware::{Logger, NormalizePath},
    App, HttpServer,
};
use tracing::info;

pub async fn run() -> std::io::Result<()> {
    // Load config once at startup
    let config = Config::load()?;
    let pool = config.pool().clone();
    let host = config.host.clone();
    let port = config.port;

    let server = HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(Logger::default())
            .wrap(RequestId::new())
            .wrap(SecurityHeaders::new())
            .wrap(NormalizePath::trim())
            // State
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(config.clone()))
            // Routes
            .configure(routes::configure_routes)
    })
    .bind((host.clone(), port))?;

    info!("Server listening on {}:{}", host, port);
    
    server.run().await
}