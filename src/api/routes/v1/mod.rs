mod organizations;
mod health;

use actix_web::Scope;

pub fn v1_routes() -> Scope {
    use actix_web::web;

    web::scope("/v1")
        .service(organizations::routes())
        .service(health::routes())
}
