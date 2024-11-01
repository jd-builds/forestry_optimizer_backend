use crate::db::{
    models::Organization,
    repositories::{
        traits::{OrganizationRepository, Repository},
        OrganizationRepositoryImpl,
    },
};
use crate::errors::AppResult;
use crate::{
    api::types::{
        organization::{CreateOrganizationInput, UpdateOrganizationInput},
        pagination::PaginationParams,
    },
    errors::AppError,
};
use diesel::PgConnection;
use uuid::Uuid;

pub struct OrganizationService {
    repository: OrganizationRepositoryImpl,
}

impl Default for OrganizationService {
    fn default() -> Self {
        Self {
            repository: OrganizationRepositoryImpl,
        }
    }
}

impl OrganizationService {
    pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> AppResult<Organization> {
        let service = Self::default();
        service.repository.find_by_id(conn, id)
    }

    pub fn list(
        conn: &mut PgConnection,
        pagination: &PaginationParams,
    ) -> AppResult<Vec<Organization>> {
        let service = Self::default();
        service.repository.list(conn, pagination)
    }

    pub fn create(
        conn: &mut PgConnection,
        input: CreateOrganizationInput,
    ) -> AppResult<Organization> {
        let service = Self::default();

        // Validate input
        if input.name.trim().is_empty() {
            return Err(AppError::validation("Organization name cannot be empty"));
        }

        // Check for existing organization with same name
        if service
            .repository
            .find_by_name(conn, &input.name)?
            .is_some()
        {
            return Err(AppError::conflict(format!(
                "Organization with name '{}' already exists",
                input.name
            )));
        }

        let organization: Organization = input.into();
        service.repository.create(conn, &organization)
    }

    pub fn update(
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrganizationInput,
    ) -> AppResult<Organization> {
        let service = Self::default();

        // Validate input
        if input.name.trim().is_empty() {
            return Err(AppError::validation("Organization name cannot be empty"));
        }

        // Check for existing organization with same name (excluding current organization)
        if let Some(existing) = service.repository.find_by_name(conn, &input.name)? {
            if existing.id != id {
                return Err(AppError::validation(format!(
                    "Organization with name '{}' already exists",
                    input.name
                )));
            }
        }

        let mut organization = service.repository.find_by_id(conn, id)?;
        organization.name = input.name;
        organization.updated_at = chrono::Utc::now();

        service.repository.update(conn, id, &organization)
    }

    pub fn delete(conn: &mut PgConnection, id: Uuid) -> AppResult<Organization> {
        let service = Self::default();
        service.repository.soft_delete(conn, id)
    }
}
