use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(crate::api::resources::auth::handlers::login))
            .route("/register", web::post().to(crate::api::resources::auth::handlers::register))
            .route("/refresh", web::post().to(crate::api::resources::auth::handlers::refresh))
    );
} 