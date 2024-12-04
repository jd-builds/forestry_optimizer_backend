use crate::{
    api::types::pagination::PaginationParams,
    error::Result,
};
use diesel::PgConnection;
use uuid::Uuid;

pub trait Repository<M> {
    fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<M>;
    fn create(&self, conn: &mut PgConnection, model: &M) -> Result<M>;
    fn update(&self, conn: &mut PgConnection, id: Uuid, model: &M) -> Result<M>;
    fn soft_delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<M>;
    fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<M>>;
}
