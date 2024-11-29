use actix_web::web;
use tracing::info;
use crate::application::api::handlers::v1::auth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring auth routes at /auth");
    cfg.route("/register", web::post().to(auth::register))
       .route("/login", web::post().to(auth::login));
} 