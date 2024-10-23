use diesel::prelude::*;
use diesel::Connection;
use uuid::Uuid;
use crate::api::CreateOrganizationInput;
use crate::models::Organization;
use crate::schema::organizations;
use crate::error::{AppResult, AppError};
use chrono::Utc;

pub fn get_organization_by_id(conn: &mut PgConnection, organization_id: Uuid) -> AppResult<Organization> {
    organizations::table
        .find(organization_id)
        .first(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::NotFoundError(format!("Organization with id {} not found", organization_id)),
            _ => e.into(),
        })
}

pub fn create_organization(conn: &mut PgConnection, input: &CreateOrganizationInput) -> AppResult<Organization> {
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

pub fn update_organization(conn: &mut PgConnection, organization_id: Uuid, updated_organization: &Organization) -> AppResult<Organization> {
    diesel::update(organizations::table.find(organization_id))
        .set(updated_organization)
        .get_result(conn)
        .map_err(Into::into)
}

pub fn delete_organization(conn: &mut PgConnection, organization_id: Uuid) -> AppResult<usize> {
    diesel::delete(organizations::table.find(organization_id))
        .execute(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::NotFoundError(format!("Organization with id {} not found", organization_id)),
            _ => AppError::DatabaseError(e),
        })
}
