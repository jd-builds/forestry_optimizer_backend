use crate::api::organization_api;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    organization_api::get_organization,
    organization_api::create_organization,
    organization_api::update_organization,
    organization_api::delete_organization,
    organization_api::list_organizations
))]
pub struct ApiDoc;
