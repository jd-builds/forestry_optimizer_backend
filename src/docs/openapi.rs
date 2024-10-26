use crate::api::organization;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    organization::get_organization,
    organization::create_organization,
    organization::update_organization,
    organization::delete_organization,
    organization::list_organizations
))]
pub struct ApiDoc;
