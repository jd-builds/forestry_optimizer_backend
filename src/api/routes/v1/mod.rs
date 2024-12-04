//! Version 1 of the API routes
//! 
//! This module contains all routes for version 1 of the API.
//! Routes are organized by resource type and protected by
//! appropriate middleware.

use actix_web::web;
use crate::api::middleware::{
    auth::{RequireAuth, Auth},
    rate_limit::RateLimit,
    request_id::RequestId,
    security::SecurityHeaders,
};

mod auth;
mod health;
mod organizations;

/// Configures all v1 API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Public routes
    cfg.service(
        web::scope("/v1")
            .wrap(SecurityHeaders::new())
            .wrap(RequestId::new())
            .wrap(RateLimit::new(100, 60)) // 100 requests per minute
            .service(health::routes())
            .service(auth::routes())
            .service(
                web::scope("/organizations")
                    // Public organization endpoints
                    .route("", web::post().to(crate::api::handlers::organization::create::create_organization))
                    // Protected organization endpoints
                    .service(
                        web::scope("")
                            .wrap(RequireAuth)
                            .wrap(Auth::new())
                            .service(organizations::routes())
                    )
            )
    );
}
