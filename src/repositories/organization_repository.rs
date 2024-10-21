use diesel::prelude::*;
use diesel::result::Error;
use diesel::Connection;
use uuid::Uuid;
use crate::api::CreateOrganizationInput;
use crate::models::Organization;
use crate::schema::organizations;
use chrono::Utc;

pub fn get_organization_by_id(conn: &mut PgConnection, organization_id: Uuid) -> QueryResult<Organization> {
    organizations::table.find(organization_id).first(conn)
}

pub fn create_organization(conn: &mut PgConnection, input: &CreateOrganizationInput) -> Result<Organization, Error> {
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

pub fn update_organization(conn: &mut PgConnection, organization_id: Uuid, updated_organization: &Organization) -> QueryResult<Organization> {
    diesel::update(organizations::table.find(organization_id))
        .set(updated_organization)
        .get_result(conn)
}

pub fn delete_organization(conn: &mut PgConnection, organization_id: Uuid) -> QueryResult<usize> {
    diesel::delete(organizations::table.find(organization_id)).execute(conn)
}
