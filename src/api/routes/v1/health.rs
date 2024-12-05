use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(crate::api::handlers::health::health_check))
            .route("/live", web::get().to(crate::api::handlers::health::liveness))
            .route("/ready", web::get().to(crate::api::handlers::health::readiness))
    );
} 