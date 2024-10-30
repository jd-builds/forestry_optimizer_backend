use chrono::{DateTime, Utc};
use diesel::{pg::Pg, prelude::*, QueryDsl};
use uuid::Uuid;

pub trait Timestamps {
    #[allow(dead_code)]
    fn created_at(&self) -> DateTime<Utc>;
    #[allow(dead_code)]
    fn updated_at(&self) -> DateTime<Utc>;
    #[allow(dead_code)]
    fn deleted_at(&self) -> Option<DateTime<Utc>>;
}

pub trait BaseModel: Sized + Timestamps {
    type Table: Table + QueryDsl;

    #[allow(dead_code)]
    fn id(&self) -> Uuid;
    #[allow(dead_code)]
    fn table() -> Self::Table;

    fn base_query() -> Box<dyn BoxableExpression<Self::Table, Pg, SqlType = diesel::sql_types::Bool>>
    where
        Self::Table: QueryDsl,
    {
        Box::new(diesel::dsl::sql::<diesel::sql_types::Bool>("TRUE"))
    }
}
