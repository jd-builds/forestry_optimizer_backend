use crate::{api::types::pagination::PaginationParams, errors::AppResult};
use diesel::PgConnection;
use uuid::Uuid;

use crate::db::models::base::BaseModel;
pub trait BaseRepository<M: BaseModel> {
    fn find_by_id(conn: &mut PgConnection, id: Uuid) -> AppResult<M>;
    fn create(conn: &mut PgConnection, model: &M) -> AppResult<M>;
    fn update(conn: &mut PgConnection, id: Uuid, model: &M) -> AppResult<M>;
    fn soft_delete(conn: &mut PgConnection, id: Uuid) -> AppResult<M>;
    fn list(conn: &mut PgConnection, pagination: &PaginationParams) -> AppResult<Vec<M>>;
}
