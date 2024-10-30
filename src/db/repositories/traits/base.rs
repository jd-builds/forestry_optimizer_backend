use crate::{api::types::pagination::PaginationParams, errors::AppResult};
use diesel::PgConnection;
use uuid::Uuid;

pub trait Repository<M> {
    fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> AppResult<M>;
    fn create(&self, conn: &mut PgConnection, model: &M) -> AppResult<M>;
    fn update(&self, conn: &mut PgConnection, id: Uuid, model: &M) -> AppResult<M>;
    fn soft_delete(&self, conn: &mut PgConnection, id: Uuid) -> AppResult<M>;
    fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> AppResult<Vec<M>>;
}
