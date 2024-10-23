use crate::config::Config;
use crate::db::DbPool;
use crate::routes;
use actix_web::{web, App, HttpServer};

pub async fn run(config: Config, pool: DbPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::api_routes())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
