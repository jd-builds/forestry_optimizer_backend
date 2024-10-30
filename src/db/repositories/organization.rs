use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::{
    api::types::pagination::PaginationParams,
    db::{
        models::{base::BaseModel, organization::Organization},
        repositories::base::BaseRepository,
        schema::organizations,
    },
};

pub struct OrganizationRepository;

impl BaseRepository<Organization> for OrganizationRepository {
    fn find_by_id(conn: &mut PgConnection, id: Uuid) -> AppResult<Organization> {
        Organization::table()
            .find(id)
            .filter(Organization::base_query())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Organization with id {} not found", id))
                }
                _ => e.into(),
            })
    }

    fn create(conn: &mut PgConnection, org: &Organization) -> AppResult<Organization> {
        diesel::insert_into(Organization::table())
            .values(org)
            .get_result(conn)
            .map_err(Into::into)
    }

    fn update(conn: &mut PgConnection, id: Uuid, org: &Organization) -> AppResult<Organization> {
        diesel::update(Organization::table().find(id))
            .set(org)
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Organization with id {} not found", id))
                }
                _ => e.into(),
            })
    }

    fn soft_delete(conn: &mut PgConnection, id: Uuid) -> AppResult<Organization> {
        diesel::update(Organization::table().find(id))
            .set(organizations::deleted_at.eq(Some(Utc::now())))
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Organization with id {} not found", id))
                }
                _ => e.into(),
            })
    }

    fn list(
        conn: &mut PgConnection,
        pagination: &PaginationParams,
    ) -> AppResult<Vec<Organization>> {
        let offset = (pagination.page - 1) * pagination.per_page;

        Organization::table()
            .filter(Organization::base_query())
            .order(organizations::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<Organization>(conn)
            .map_err(Into::into)
    }
}

// Helper methods specific to Organization
impl OrganizationRepository {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::organization::{CreateOrganizationInput, UpdateOrganizationInput};
    use diesel::{pg::PgConnection, Connection};
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
        let result = OrganizationRepository::find_by_id(conn, org_id);
        assert!(result.is_err()); // Assuming the ID doesn't exist
    }

    #[test]
    fn test_create_organization() {
        let conn = &mut establish_connection();
        let input = CreateOrganizationInput {
            name: "Test Org".to_string(),
        };
        let result = OrganizationRepository::create(conn, &input.into());
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
        let org = OrganizationRepository::create(conn, &input.into()).unwrap();
        let updated_name = "Updated Org";
        let result = OrganizationRepository::update(
            conn,
            org.id,
            &UpdateOrganizationInput {
                name: updated_name.to_string(),
            }
            .into(),
        );
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
        let org = OrganizationRepository::create(conn, &input.into()).unwrap();
        let result = OrganizationRepository::soft_delete(conn, org.id);
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
        OrganizationRepository::create(conn, &input1.into()).unwrap();
        OrganizationRepository::create(conn, &input2.into()).unwrap();

        let result = OrganizationRepository::list(
            conn,
            &PaginationParams {
                page: 1,
                per_page: 10,
            },
        );
        assert!(result.is_ok());
        let orgs = result.unwrap();
        assert!(orgs.len() >= 2);
    }
}
