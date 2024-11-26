use crate::{
    db::models::Organization,
    errors::Result,
};
use diesel::PgConnection;

use super::base::Repository;

pub trait OrganizationRepository: Repository<Organization> {
    fn find_by_name(&self, conn: &mut PgConnection, name: &str) -> Result<Option<Organization>>;
}
