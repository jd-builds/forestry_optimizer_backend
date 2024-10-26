use crate::api::handlers::organization::{create, delete, read, update};
use actix_web::{web, Scope};

pub fn api_routes() -> Scope {
    web::scope("/api").service(
        web::scope("/organizations")
            .route("", web::post().to(create::create_organization))
            .route("/{id}", web::get().to(read::get_organization))
            .route("/{id}", web::put().to(update::update_organization))
            .route("/{id}", web::delete().to(delete::delete_organization))
            .route("", web::get().to(read::list_organizations)),
    )
}
