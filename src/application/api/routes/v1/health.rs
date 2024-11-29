use actix_web::web;
use tracing::info;
use crate::application::api::handlers::v1::health;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring health routes");
    cfg.route("/check", web::get().to(health::health_check));
} 