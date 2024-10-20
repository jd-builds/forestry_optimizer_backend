use actix_web::{web, App, HttpServer};
use crate::config::Config;
use crate::db::DbPool;
use crate::routes;

pub async fn run(config: Config, pool: DbPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::hello)
            // Add more services here
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
