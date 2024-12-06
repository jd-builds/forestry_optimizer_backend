use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use super::openapi::ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
            .config(utoipa_swagger_ui::Config::new(["/api-docs/openapi.json"]))
    );
}