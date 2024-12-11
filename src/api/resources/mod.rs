use actix_web::web;

use super::middleware;
mod health;
pub mod auth;
pub mod organization;
pub mod docs;

/// Configures all application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(configure_v1_routes)
    );
}

/// Configures all v1 API routes
fn configure_v1_routes(cfg: &mut web::ServiceConfig) {
    use middleware::{
        rate_limit::RateLimit,
        request_id::RequestId,
        security::SecurityHeaders,
    };

    cfg.service(
        web::scope("/v1")
            .wrap(SecurityHeaders::new())
            .wrap(RequestId::new())
            .wrap(RateLimit::new(100, 60)) // 100 requests per minute
            .configure(health::routes::configure)
            .configure(auth::routes::configure)
            .configure(organization::routes::configure)
            .configure(docs::configure)  // Moved docs into resources
    );
}