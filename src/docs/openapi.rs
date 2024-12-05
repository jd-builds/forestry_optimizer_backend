//! OpenAPI documentation generation
//! 
//! This module handles the generation of OpenAPI documentation
//! for all API endpoints.

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::organization::read::get_organization,
        crate::api::handlers::organization::read::list_organizations,
        crate::api::handlers::organization::create::create_organization,
        crate::api::handlers::organization::update::update_organization,
        crate::api::handlers::organization::delete::delete_organization
    ),
    components(
        schemas(
            crate::api::models::organization::CreateOrganizationInput,
            crate::api::models::organization::UpdateOrganizationInput,
            crate::api::models::organization::OrganizationDto,
            crate::api::models::pagination::PaginationParams,
            crate::api::models::pagination::PaginatedResponse<crate::api::models::organization::OrganizationDto>,
            crate::api::models::responses::ApiResponse<crate::api::models::organization::OrganizationDto>,
            crate::api::models::responses::ErrorResponse
        )
    ),
    tags(
        (name = "organizations", description = "Organization management endpoints")
    )
)]
pub struct ApiDoc;
