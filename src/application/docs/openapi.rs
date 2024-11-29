use crate::application::api::handlers::v1::organization as org;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    org::create_organization,
    org::get_organization,
    org::list_organizations,
    org::update_organization,
    org::delete_organization
))]
pub struct ApiDoc; 