use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(crate::api::handlers::auth::login))
            .route("/register", web::post().to(crate::api::handlers::auth::register))
            .route("/refresh", web::post().to(crate::api::handlers::auth::refresh))
    );
} 