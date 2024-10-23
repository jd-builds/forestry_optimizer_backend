use crate::api::CreateOrganizationInput;
use crate::error::{AppError, AppResult};
use crate::models::Organization;
use crate::schema::organizations;
use chrono::Utc;
use diesel::prelude::*;
use diesel::Connection;
use uuid::Uuid;

pub fn get_organization_by_id(
    conn: &mut PgConnection,
    organization_id: Uuid,
) -> AppResult<Organization> {
    organizations::table
        .find(organization_id)
        .filter(organizations::deleted_at.is_null())
        .first(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::NotFound(format!(
                "Organization with id {} not found",
                organization_id
            )),
            _ => e.into(),
        })
}

pub fn create_organization(
    conn: &mut PgConnection,
    input: &CreateOrganizationInput,
) -> AppResult<Organization> {
    conn.transaction(|conn| {
        let new_organization = Organization {
            id: Uuid::new_v4(),
            name: input.name.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        diesel::insert_into(organizations::table)
            .values(&new_organization)
            .execute(conn)?;

        Ok(new_organization)
    })
}

pub fn update_organization(
    conn: &mut PgConnection,
    organization_id: Uuid,
    name: &str,
) -> AppResult<Organization> {
    diesel::update(organizations::table.find(organization_id))
        .set((
            organizations::name.eq(name),
            organizations::updated_at.eq(Utc::now()),
        ))
        .get_result(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::NotFound(format!(
                "Organization with id {} not found",
                organization_id
            )),
            _ => e.into(),
        })
}

pub fn delete_organization(
    conn: &mut PgConnection,
    organization_id: Uuid,
) -> AppResult<Organization> {
    diesel::update(organizations::table.find(organization_id))
        .set(organizations::deleted_at.eq(Some(Utc::now())))
        .get_result(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::NotFound(format!(
                "Organization with id {} not found",
                organization_id
            )),
            _ => e.into(),
        })
}

pub fn list_organizations(
    conn: &mut PgConnection,
    limit: i64,
    offset: i64,
) -> AppResult<Vec<Organization>> {
    organizations::table
        .filter(organizations::deleted_at.is_null())
        .order(organizations::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<Organization>(conn)
        .map_err(Into::into)
}
