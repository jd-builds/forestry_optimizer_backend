use crate::{db::models::Organization, errors::AppResult};
use diesel::PgConnection;

use super::base::Repository;

pub trait OrganizationRepository: Repository<Organization> {
    // Add any organization-specific repository methods here
    fn find_by_name(&self, conn: &mut PgConnection, name: &str) -> AppResult<Option<Organization>>;
}
