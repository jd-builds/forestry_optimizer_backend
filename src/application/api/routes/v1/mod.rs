pub mod auth;
pub mod health;
pub mod organizations;

use actix_web::web;
use tracing::info;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring API v1 routes");
    
    let api_v1 = web::scope("/api/v1")
        .service(
            web::scope("/health")
                .configure(health::configure)
        );

    cfg.service(api_v1);
} 