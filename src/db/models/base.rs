use chrono::{DateTime, Utc};
use diesel::{pg::Pg, prelude::*, QueryDsl};
use uuid::Uuid;
use crate::errors::Result;

/// Trait for handling timestamp fields common across all models
#[allow(unused)]
pub trait Timestamps {
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn deleted_at(&self) -> Option<DateTime<Utc>>;
    fn is_deleted(&self) -> bool {
        self.deleted_at().is_some()
    }
}

/// Base model trait providing common functionality for all database models
#[allow(dead_code)]
pub trait BaseModel: Sized + Timestamps {
    type Table: Table + QueryDsl;

    fn id(&self) -> Uuid;
    fn table() -> Self::Table;
    
    fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Self>
    where
        Self: Queryable<diesel::sql_types::Record<(
            diesel::sql_types::Uuid,
            diesel::sql_types::Text,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Nullable<diesel::sql_types::Timestamptz>,
        )>, Pg>;
        
    fn soft_delete(&mut self) {
        self.set_deleted_at(Some(Utc::now()));
    }
    
    fn set_deleted_at(&mut self, timestamp: Option<DateTime<Utc>>);

    fn base_query() -> Box<dyn BoxableExpression<Self::Table, Pg, SqlType = diesel::sql_types::Bool>> {
        Box::new(diesel::dsl::sql::<diesel::sql_types::Bool>("TRUE"))
    }
}
