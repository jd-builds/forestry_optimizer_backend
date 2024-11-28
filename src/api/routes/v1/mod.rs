mod organizations;
mod health;
pub mod auth;

use actix_web::Scope;

/// Protected routes that require authentication
pub fn v1_routes() -> Scope {
    use actix_web::web;

    web::scope("")
        .service(organizations::routes())
        .service(health::routes())
}
