use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::resources::health::handlers::health_check,
        crate::api::resources::health::handlers::liveness,
        crate::api::resources::health::handlers::readiness,
        crate::api::resources::auth::handlers::login,
        crate::api::resources::auth::handlers::register,
        crate::api::resources::auth::handlers::refresh,
        crate::api::resources::organization::handlers::read::get_organization,
        crate::api::resources::organization::handlers::read::list_organizations,
        crate::api::resources::organization::handlers::create::create_organization,
        crate::api::resources::organization::handlers::update::update_organization,
        crate::api::resources::organization::handlers::delete::delete_organization
    ),
    components(
        schemas(
            crate::api::resources::auth::dto::LoginRequest,
            crate::api::resources::auth::dto::RegisterRequest,
            crate::api::resources::auth::dto::RefreshRequest,
            crate::api::resources::auth::dto::AuthResponse,
            crate::api::resources::auth::dto::UserResponse,
            crate::api::resources::health::dto::HealthStatus,
            crate::api::resources::health::dto::SystemMetrics,
            crate::api::resources::organization::dto::CreateOrganizationInput,
            crate::api::resources::organization::dto::UpdateOrganizationInput,
            crate::api::resources::organization::dto::OrganizationResponse,
            crate::api::utils::PaginationParams,
            crate::api::utils::PaginatedResponse<crate::api::resources::organization::dto::OrganizationResponse>,
            crate::api::utils::ApiResponse<crate::api::resources::organization::dto::OrganizationResponse>,
            crate::api::utils::ErrorResponse
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "organizations", description = "Organization management endpoints")
    )
)]
pub struct ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
            .config(utoipa_swagger_ui::Config::new(["/api-docs/openapi.json"]))
    );
} 
