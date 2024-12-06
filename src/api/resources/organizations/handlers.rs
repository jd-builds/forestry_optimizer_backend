//! Organization resource handlers
//! 
//! This module contains all the handlers for the organization resource endpoints.
//! It follows RESTful principles and provides CRUD operations.

use crate::{
    api::utils::ApiResponseBuilder,
    api::resources::organizations::dto::{
        CreateOrganizationInput, OrganizationResponse, UpdateOrganizationInput, Validate,
    },
    database::{get_connection, models::Organization, repositories::OrganizationRepositoryImpl, DbPool},
    error::ApiError,
    domain::OrganizationService,
};
use actix_web::{web, HttpResponse};
use uuid::Uuid;

/// Handler context containing shared resources and dependencies
/// 
/// This struct encapsulates common functionality and dependencies
/// used across all organization handlers.
struct HandlerContext {
    pool: web::Data<DbPool>,
    service: OrganizationService<OrganizationRepositoryImpl>,
}

impl HandlerContext {
    /// Creates a new handler context with the given database pool
    #[inline]
    fn new(pool: web::Data<DbPool>) -> Self {
        Self {
            pool,
            service: OrganizationService::new(OrganizationRepositoryImpl),
        }
    }
}

pub mod read {
    use crate::api::{utils::{ApiResponseBuilder, PaginatedResponse, PaginationParams}, resources::organizations::dto::ListOrganizationsQuery};

    use super::*;

    /// Retrieves a single organization by ID
    /// 
    /// # OpenAPI Specification
    #[utoipa::path(
        get,
        path = "/v1/organizations/{id}",
        responses(
            (status = 200, description = "Organization found", body = OrganizationResponse),
            (status = 404, description = "Organization not found"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = Uuid, Path, description = "Organization ID")
        )
    )]
    pub async fn get_organization(
        pool: web::Data<DbPool>,
        organization_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, ApiError> {
        let ctx = HandlerContext::new(pool);
        let org_id = *organization_id;

        let mut conn = get_connection(&ctx.pool)?;
        let organization = ctx.service.get(&mut conn, org_id).await?;

        Ok(HttpResponse::Ok().json(
            ApiResponseBuilder::success()
                .with_message("Organization retrieved successfully")
                .with_data(OrganizationResponse {
                    id: organization.id,
                    name: organization.name,
                    created_at: organization.created_at,
                    updated_at: organization.updated_at,
                })
                .build()
        ))
    }

    /// Lists all organizations with pagination
    /// 
    /// # OpenAPI Specification
    #[utoipa::path(
        get,
        path = "/v1/organizations",
        responses(
            (status = 200, description = "List of organizations", body = PaginatedResponse<Organization>),
            (status = 400, description = "Bad request"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("limit" = Option<i64>, Query, description = "Number of items per page"),
            ("offset" = Option<i64>, Query, description = "Number of items to skip")
        )
    )]
    pub async fn list_organizations(
        pool: web::Data<DbPool>,
        query: web::Query<ListOrganizationsQuery>,
    ) -> Result<HttpResponse, ApiError> {
        let ctx = HandlerContext::new(pool);
        let pagination = PaginationParams {
            page: (query.page.unwrap_or(0) / query.per_page.unwrap_or(10)) + 1,
            per_page: query.per_page.unwrap_or(10),
        };

        let mut conn = get_connection(&ctx.pool)?;
        let organizations = ctx.service.list(&mut conn, &pagination).await?;

        Ok(HttpResponse::Ok().json(
            ApiResponseBuilder::success()
                .with_message("Organizations retrieved successfully")
                .with_data(PaginatedResponse::new(
                    organizations.clone(),
                    organizations.len() as i64,
                    &pagination
                ))
                .build()
        ))
    }
}

pub mod create {
    use super::*;

    /// Creates a new organization
    /// 
    /// # OpenAPI Specification
    #[utoipa::path(
        post,
        path = "/v1/organizations",
        request_body = CreateOrganizationInput,
        responses(
            (status = 201, description = "Organization created", body = OrganizationResponse),
            (status = 400, description = "Bad request"),
            (status = 409, description = "Organization already exists"),
            (status = 500, description = "Internal server error")
        )
    )]
    pub async fn create_organization(
        pool: web::Data<DbPool>,
        new_organization: web::Json<CreateOrganizationInput>,
    ) -> Result<HttpResponse, ApiError> {
        let ctx = HandlerContext::new(pool);
        let input = new_organization.into_inner();

        let mut conn = get_connection(&ctx.pool)?;
        input.validate(&mut conn, ctx.service.repository(), None).await?;
        let organization = ctx.service.create(&mut conn, input).await?;

        Ok(HttpResponse::Created().json(
            ApiResponseBuilder::success()
                .with_message("Organization created successfully")
                .with_data(OrganizationResponse {
                    id: organization.id,
                    name: organization.name,
                    created_at: organization.created_at,
                    updated_at: organization.updated_at,
                })
                .build()
        ))
    }
}

pub mod update {
    use super::*;

    /// Updates an existing organization
    /// 
    /// # OpenAPI Specification
    #[utoipa::path(
        put,
        path = "/v1/organizations/{id}",
        request_body = UpdateOrganizationInput,
        responses(
            (status = 200, description = "Organization updated", body = OrganizationResponse),
            (status = 400, description = "Bad request"),
            (status = 404, description = "Organization not found"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = Uuid, Path, description = "Organization ID")
        )
    )]
    pub async fn update_organization(
        pool: web::Data<DbPool>,
        organization_id: web::Path<Uuid>,
        updated_organization: web::Json<UpdateOrganizationInput>,
    ) -> Result<HttpResponse, ApiError> {
        let ctx = HandlerContext::new(pool);
        let org_id = *organization_id;
        let input = updated_organization.into_inner();

        let mut conn = get_connection(&ctx.pool)?;
        input.validate(&mut conn, ctx.service.repository(), Some(org_id)).await?;
        let organization = ctx.service.update(&mut conn, org_id, input).await?;

        Ok(HttpResponse::Ok().json(
            ApiResponseBuilder::success()
                .with_message("Organization updated successfully")
                .with_data(OrganizationResponse {
                    id: organization.id,
                    name: organization.name,
                    created_at: organization.created_at,
                    updated_at: organization.updated_at,
                })
                .build()
        ))
    }
}

pub mod delete {
    use super::*;

    /// Soft deletes an organization
    /// 
    /// # OpenAPI Specification
    #[utoipa::path(
        delete,
        path = "/v1/organizations/{id}",
        responses(
            (status = 204, description = "Organization deleted"),
            (status = 404, description = "Organization not found"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("id" = Uuid, Path, description = "Organization ID")
        )
    )]
    pub async fn delete_organization(
        pool: web::Data<DbPool>,
        organization_id: web::Path<Uuid>,
    ) -> Result<HttpResponse, ApiError> {
        let ctx = HandlerContext::new(pool);
        let org_id = *organization_id;

        let mut conn = get_connection(&ctx.pool)?;
        ctx.service.delete(&mut conn, org_id).await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
