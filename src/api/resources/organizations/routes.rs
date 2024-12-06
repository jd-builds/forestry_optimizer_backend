use actix_web::web;
use crate::api::middleware::auth::{Auth, RequireAuth};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/organizations", web::post().to(crate::api::resources::organizations::handlers::create::create_organization))
        .service(
            web::scope("/organizations")
                .wrap(RequireAuth)
                .wrap(Auth::new())
                .route("", web::get().to(crate::api::resources::organizations::handlers::read::list_organizations))
                .route("/{id}", web::get().to(crate::api::resources::organizations::handlers::read::get_organization))
                .route("/{id}", web::put().to(crate::api::resources::organizations::handlers::update::update_organization))
                .route("/{id}", web::delete().to(crate::api::resources::organizations::handlers::delete::delete_organization))
        );
} 