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
            crate::api::types::organization::CreateOrganizationInput,
            crate::api::types::organization::UpdateOrganizationInput,
            crate::api::types::organization::OrganizationDto,
            crate::api::types::pagination::PaginationParams,
            crate::api::types::pagination::PaginatedResponse<crate::api::types::organization::OrganizationDto>,
            crate::api::types::responses::ApiResponse<crate::api::types::organization::OrganizationDto>,
            crate::api::types::responses::ErrorResponse
        )
    ),
    tags(
        (name = "organizations", description = "Organization management endpoints")
    )
)]
pub struct ApiDoc;
