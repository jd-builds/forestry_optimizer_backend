//! API route configuration
//! 
//! This module contains all route definitions and configurations
//! for the application's HTTP API.

pub mod v1;

use actix_web::web;
use tracing::info;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring root routes");
    cfg.service(
        web::scope("/api/v1")
            .configure(v1::configure)
    );
} 