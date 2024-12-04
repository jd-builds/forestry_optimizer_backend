//! API documentation module
//! 
//! This module handles the OpenAPI documentation generation and
//! serves the Swagger UI interface.

use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod openapi;

pub use openapi::ApiDoc;

/// Configures the documentation routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
    );
}
