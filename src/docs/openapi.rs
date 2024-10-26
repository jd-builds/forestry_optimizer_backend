use crate::api::handlers::organization;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    organization::read::get_organization,
    organization::read::list_organizations,
    organization::create::create_organization,
    organization::update::update_organization,
    organization::delete::delete_organization
))]
pub struct ApiDoc;
