use actix_web::web;
use crate::api::middleware::auth::{Auth, RequireAuth};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/organizations", web::post().to(crate::api::handlers::organization::create::create_organization))
        .service(
            web::scope("/organizations")
                .wrap(RequireAuth)
                .wrap(Auth::new())
                .route("", web::get().to(crate::api::handlers::organization::read::list_organizations))
                .route("/{id}", web::get().to(crate::api::handlers::organization::read::get_organization))
                .route("/{id}", web::put().to(crate::api::handlers::organization::update::update_organization))
                .route("/{id}", web::delete().to(crate::api::handlers::organization::delete::delete_organization))
        );
} 