use crate::api::organization;
use actix_web::{web, Scope};

pub fn api_routes() -> Scope {
    web::scope("/api").service(
        web::scope("/organizations")
            .route("", web::post().to(organization::create_organization))
            .route("/{id}", web::get().to(organization::get_organization))
            .route("/{id}", web::put().to(organization::update_organization))
            .route("/{id}", web::delete().to(organization::delete_organization))
            .route("", web::get().to(organization::list_organizations)),
    )
}
