//! Route configuration and setup
//! 
//! This module handles the configuration of all API routes and
//! their organization into different versions and scopes.

use actix_web::web;
mod v1;

/// Configures all application routes
/// 
/// This function sets up all API routes, including:
/// - API versioning (v1, etc.)
/// - OpenAPI documentation
/// - Health checks
/// - Authentication routes
/// - Protected routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("")
                .configure(v1::configure_routes)
        )
        .service(
            web::scope("/docs")
                .configure(crate::docs::configure_routes)
        );
}