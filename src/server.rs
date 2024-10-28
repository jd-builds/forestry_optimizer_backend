use crate::api::routes;
use crate::config::Config;
use crate::docs::openapi::ApiDoc;
use crate::middleware::Logging;
use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run(config: Config) -> std::io::Result<()> {
    // Create shared state
    let pool = web::Data::new(config.pool().clone());
    let host = config.host.clone();
    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .wrap(Logging)
            .app_data(pool.clone())
            .service(routes::v1_routes())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
