use actix_web::web;
use tracing::info;
use crate::application::api::handlers::v1::organization::{
    create_organization,
    get_organization,
    list_organizations,
    update_organization,
    delete_organization,
};
use crate::application::api::middleware::auth::role::RequireRole;
use crate::domain::models::user::Role;

pub fn configure(cfg: &mut web::ServiceConfig) {
    info!("Configuring organization routes at /api/v1/organizations");
    cfg.service(
        web::scope("/organizations")
            .route("", web::post().to(create_organization))
            .route("", web::get().to(list_organizations))
            .route("/{id}", web::get().to(get_organization))
            .route(
                "/{id}",
                web::put()
                    .to(update_organization)
                    .wrap(RequireRole(Role::Admin)),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(delete_organization)
                    .wrap(RequireRole(Role::Admin)),
            )
    );
} 