use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use crate::api::doc::ApiDoc;
use crate::config::Config;
use crate::db::DbPool;
use crate::routes;
use actix_web::{web, App, HttpServer};

pub async fn run(config: Config, pool: DbPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::api_routes())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi())
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
