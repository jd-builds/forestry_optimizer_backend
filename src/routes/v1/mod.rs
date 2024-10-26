mod organizations;

use actix_web::{web, Scope};

pub fn v1_routes() -> Scope {
    web::scope("/v1").service(organizations::routes())
}
