use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(crate::api::resources::health::handlers::health_check))
            .route("/live", web::get().to(crate::api::resources::health::handlers::liveness))
            .route("/ready", web::get().to(crate::api::resources::health::handlers::readiness))
    );
} 