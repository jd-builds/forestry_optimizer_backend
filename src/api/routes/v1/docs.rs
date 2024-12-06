use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::health::health_check,
        crate::api::handlers::health::liveness,
        crate::api::handlers::health::readiness,
        crate::api::handlers::auth::login,
        crate::api::handlers::auth::register,
        crate::api::handlers::auth::refresh,
        crate::api::handlers::organization::read::get_organization,
        crate::api::handlers::organization::read::list_organizations,
        crate::api::handlers::organization::create::create_organization,
        crate::api::handlers::organization::update::update_organization,
        crate::api::handlers::organization::delete::delete_organization
    ),
    components(
        schemas(
            crate::api::dto::auth::LoginRequest,
            crate::api::dto::auth::RegisterRequest,
            crate::api::dto::auth::RefreshRequest,
            crate::api::dto::auth::AuthResponse,
            crate::api::dto::auth::UserResponse,
            crate::api::dto::health::HealthStatus,
            crate::api::dto::health::SystemMetrics,
            crate::api::dto::organization::CreateOrganizationInput,
            crate::api::dto::organization::UpdateOrganizationInput,
            crate::api::dto::organization::OrganizationResponse,
            crate::api::dto::PaginationParams,
            crate::api::dto::PaginatedResponse<crate::api::dto::organization::OrganizationResponse>,
            crate::api::dto::ApiResponse<crate::api::dto::organization::OrganizationResponse>,
            crate::api::dto::ErrorResponse
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "organizations", description = "Organization management endpoints")
    )
)]
struct ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", ApiDoc::openapi())
            .config(utoipa_swagger_ui::Config::new(["/api-docs/openapi.json"]))
    );
} 
