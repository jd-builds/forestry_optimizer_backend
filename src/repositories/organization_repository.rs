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

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::pg::PgConnection;
    use dotenv::dotenv;
    use std::env;
    use uuid::Uuid;

    fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect("Error connecting to database")
    }

    #[test]
    fn test_get_organization_by_id() {
        let conn = &mut establish_connection();
        let org_id = Uuid::new_v4(); // Use an existing ID for a real test
        let result = get_organization_by_id(conn, org_id);
        assert!(result.is_err()); // Assuming the ID doesn't exist
    }

    #[test]
    fn test_create_organization() {
        let conn = &mut establish_connection();
        let input = CreateOrganizationInput {
            name: "Test Org".to_string(),
        };
        let result = create_organization(conn, &input);
        assert!(result.is_ok());
        let organization = result.unwrap();
        assert_eq!(organization.name, "Test Org");
    }

    #[test]
    fn test_update_organization() {
        let conn = &mut establish_connection();
        let input = CreateOrganizationInput {
            name: "Test Org".to_string(),
        };
        let org = create_organization(conn, &input).unwrap();
        let updated_name = "Updated Org";
        let result = update_organization(conn, org.id, updated_name);
        assert!(result.is_ok());
        let updated_org = result.unwrap();
        assert_eq!(updated_org.name, updated_name);
    }

    #[test]
    fn test_delete_organization() {
        let conn = &mut establish_connection();
        let input = CreateOrganizationInput {
            name: "Test Org".to_string(),
        };
        let org = create_organization(conn, &input).unwrap();
        let result = delete_organization(conn, org.id);
        assert!(result.is_ok());
        let deleted_org = result.unwrap();
        assert!(deleted_org.deleted_at.is_some());
    }

    #[test]
    fn test_list_organizations() {
        let conn = &mut establish_connection();
        let input1 = CreateOrganizationInput {
            name: "Org 1".to_string(),
        };
        let input2 = CreateOrganizationInput {
            name: "Org 2".to_string(),
        };
        create_organization(conn, &input1).unwrap();
        create_organization(conn, &input2).unwrap();

        let result = list_organizations(conn, 10, 0);
        assert!(result.is_ok());
        let orgs = result.unwrap();
        assert!(orgs.len() >= 2);
    }
}
