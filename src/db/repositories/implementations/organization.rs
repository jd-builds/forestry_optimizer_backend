use crate::{
    api::types::pagination::PaginationParams,
    db::{
        models::Organization,
        repositories::traits::{OrganizationRepository, Repository},
        schema::organizations::dsl::*,
    },
    errors::{AppError, AppResult},
};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub struct OrganizationRepositoryImpl;

impl Repository<Organization> for OrganizationRepositoryImpl {
    fn find_by_id(&self, conn: &mut PgConnection, search_id: Uuid) -> AppResult<Organization> {
        organizations
            .find(search_id)
            .filter(deleted_at.is_null())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::not_found(format!("Organization with id {} not found", search_id))
                }
                _ => AppError::Database(e),
            })
    }

    fn create(&self, conn: &mut PgConnection, org: &Organization) -> AppResult<Organization> {
        diesel::insert_into(organizations)
            .values(org)
            .get_result(conn)
            .map_err(AppError::Database)
    }

    fn update(
        &self,
        conn: &mut PgConnection,
        search_id: Uuid,
        org: &Organization,
    ) -> AppResult<Organization> {
        diesel::update(organizations.find(search_id))
            .set(org)
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Organization with id {} not found", search_id))
                }
                _ => e.into(),
            })
    }

    fn soft_delete(&self, conn: &mut PgConnection, search_id: Uuid) -> AppResult<Organization> {
        diesel::update(organizations.find(search_id))
            .set(deleted_at.eq(Some(Utc::now())))
            .get_result(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Organization with id {} not found", search_id))
                }
                _ => e.into(),
            })
    }

    fn list(
        &self,
        conn: &mut PgConnection,
        pagination: &PaginationParams,
    ) -> AppResult<Vec<Organization>> {
        let offset = (pagination.page - 1) * pagination.per_page;

        organizations
            .filter(deleted_at.is_null())
            .offset(offset)
            .limit(pagination.per_page)
            .load(conn)
            .map_err(Into::into)
    }
}

impl OrganizationRepository for OrganizationRepositoryImpl {
    fn find_by_name(
        &self,
        conn: &mut PgConnection,
        search_name: &str,
    ) -> AppResult<Option<Organization>> {
        organizations
            .filter(name.eq(search_name))
            .filter(deleted_at.is_null())
            .first(conn)
            .optional()
            .map_err(Into::into)
    }
}
