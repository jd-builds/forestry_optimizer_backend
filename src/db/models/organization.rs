use super::base::{BaseModel, Timestamps};
use crate::db::schema::organizations;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(
    Debug,
    Default,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
    ToSchema,
)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Timestamps for Organization {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
}

impl BaseModel for Organization {
    type Table = organizations::table;

    fn id(&self) -> Uuid {
        self.id
    }

    fn table() -> Self::Table {
        organizations::table
    }

    fn base_query() -> Box<dyn BoxableExpression<Self::Table, Pg, SqlType = diesel::sql_types::Bool>>
    {
        Box::new(organizations::deleted_at.is_null())
    }
}
