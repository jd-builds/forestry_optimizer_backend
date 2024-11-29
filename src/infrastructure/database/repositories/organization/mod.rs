//! Organization repository implementation
//! 
//! This module provides the concrete implementation of the organization repository
//! traits. It handles all database operations for organizations with proper error
//! handling and logging.

use crate::{
    common::{error::{ApiError, ErrorCode, Result}, pagination::PaginationParams},
    domain::{
        models::Organization,
        repositories::{base::Repository, OrganizationRepository},
    },
    infrastructure::database::schema::public::organizations,
};
use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, warn};
use uuid::Uuid;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
    prelude::*,
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use std::sync::Arc;

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = organizations)]
struct OrganizationModel {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
}

impl From<OrganizationModel> for Organization {
    fn from(model: OrganizationModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}

impl From<&Organization> for OrganizationModel {
    fn from(org: &Organization) -> Self {
        Self {
            id: org.id,
            name: org.name.clone(),
            created_at: org.created_at,
            updated_at: org.updated_at,
            deleted_at: org.deleted_at,
        }
    }
}

pub struct OrganizationRepositoryImpl {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl OrganizationRepositoryImpl {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self { pool }
    }

    fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        self.pool.get().map_err(|e| {
            error!(error = %e, "Failed to get database connection");
            ApiError::new(
                ErrorCode::DatabaseError,
                "Failed to get database connection",
                Default::default(),
            )
        })
    }
}

#[async_trait]
impl Repository<Organization> for OrganizationRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Organization> {
        let mut conn = self.get_conn()?;
        
        organizations::table
            .find(id)
            .filter(organizations::deleted_at.is_null())
            .first::<OrganizationModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    warn!(error_code = %ErrorCode::NotFound, organization_id = %id, "Organization not found");
                    ApiError::not_found(format!("Organization with id {} not found", id))
                }
                _ => {
                    error!(error_code = %ErrorCode::DatabaseError, organization_id = %id, error = %e, "Database error");
                    ApiError::database_error("Failed to find organization", None)
                }
            })
    }

    async fn create(&self, org: &Organization) -> Result<Organization> {
        let mut conn = self.get_conn()?;
        let model = OrganizationModel::from(org);
        
        diesel::insert_into(organizations::table)
            .values(&model)
            .get_result::<OrganizationModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error_code = %ErrorCode::DatabaseError, organization_id = %org.id, error = %e, "Database error");
                ApiError::database_error("Failed to create organization", None)
            })
    }

    async fn update(&self, id: Uuid, org: &Organization) -> Result<Organization> {
        let mut conn = self.get_conn()?;
        let model = OrganizationModel::from(org);
        
        diesel::update(organizations::table.find(id))
            .set(&model)
            .get_result::<OrganizationModel>(&mut conn)
            .map(Into::into)
            .map_err(|e| {
                error!(error_code = %ErrorCode::DatabaseError, organization_id = %id, error = %e, "Database error");
                ApiError::database_error("Failed to update organization", None)
            })
    }

    async fn soft_delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.get_conn()?;
        
        diesel::update(organizations::table.find(id))
            .set(organizations::deleted_at.eq(Some(Utc::now())))
            .execute(&mut conn)
            .map_err(|e| {
                error!(error_code = %ErrorCode::DatabaseError, organization_id = %id, error = %e, "Database error");
                ApiError::database_error("Failed to delete organization", None)
            })?;
        Ok(())
    }

    async fn list(&self, pagination: &PaginationParams) -> Result<(Vec<Organization>, i64)> {
        let mut conn = self.get_conn()?;
        
        let results = organizations::table
            .filter(organizations::deleted_at.is_null())
            .offset(pagination.offset.into())
            .limit(pagination.limit.into())
            .load::<OrganizationModel>(&mut conn)
            .map_err(|e| {
                error!(error_code = %ErrorCode::DatabaseError, error = %e, "Database error");
                ApiError::database_error("Failed to list organizations", None)
            })?;

        let total = organizations::table
            .filter(organizations::deleted_at.is_null())
            .count()
            .get_result(&mut conn)
            .map_err(|e| {
                error!(error_code = %ErrorCode::DatabaseError, error = %e, "Database error");
                ApiError::database_error("Failed to count organizations", None)
            })?;

        Ok((results.into_iter().map(Into::into).collect(), total))
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    async fn find_by_name(&self, search_name: &str) -> Result<Option<Organization>> {
        let mut conn = self.get_conn()?;
        
        organizations::table
            .filter(organizations::name.eq(search_name))
            .filter(organizations::deleted_at.is_null())
            .first::<OrganizationModel>(&mut conn)
            .optional()
            .map(|opt| opt.map(Into::into))
            .map_err(|e| {
                error!(error_code = %ErrorCode::DatabaseError, name = %search_name, error = %e, "Database error");
                ApiError::database_error("Failed to find organization by name", None)
            })
    }
}
