use crate::{
    api::types::{
        organization::{
            CreateOrganizationInput, ListOrganizationsQuery, OrganizationResponse,
            UpdateOrganizationInput, Validate,
        },
        pagination::{PaginatedResponse, PaginationParams, PaginationMeta},
        responses::ApiResponseBuilder,
    },
    db::{get_connection, models::Organization, DbPool},
    errors::ApiError,
    services::organization::OrganizationService,
};
use actix_web::{web, HttpResponse};
use tracing::error;
use uuid::Uuid;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

// Common handler utilities
struct HandlerContext {
    pool: web::Data<DbPool>,
    service: OrganizationService,
}

impl HandlerContext {
    fn new(pool: web::Data<DbPool>) -> Self {
        Self {
            pool,
            service: OrganizationService::default(),
        }
    }

    async fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ApiError> {
        get_connection(&self.pool).map_err(|e| {
            error!("Database connection error: {}", e);
            ApiError::new(
                "DATABASE_ERROR",
                "Failed to establish database connection",
                Some(serde_json::json!({ "details": e.to_string() })),
            )
        })
    }
}

pub mod read {
    use super::*;

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

        let mut conn = ctx.get_connection().await?;
        let organization = ctx.service.find_by_id(&mut conn, org_id).await?;

        Ok(HttpResponse::Ok().json(
            ApiResponseBuilder::success()
                .with_message("Organization retrieved successfully")
                .with_data(OrganizationResponse {
                    organization: organization.into(),
                })
                .build()
        ))
    }

    #[utoipa::path(
        get,
        path = "/v1/organizations",
        responses(
            (status = 200, description = "List of organizations", body = PaginatedResponse<Organization>),
            (status = 400, description = "Bad request"),
            (status = 500, description = "Internal server error")
        ),
        params(
            ("limit" = Option<i64>, Query, description = "Limit the number of organizations"),
            ("offset" = Option<i64>, Query, description = "Offset for pagination")
        )
    )]
    pub async fn list_organizations(
        pool: web::Data<DbPool>,
        query: web::Query<ListOrganizationsQuery>,
    ) -> Result<HttpResponse, ApiError> {
        let ctx = HandlerContext::new(pool);
        let pagination = PaginationParams {
            page: (query.offset.unwrap_or(0) / query.limit.unwrap_or(10)) + 1,
            per_page: query.limit.unwrap_or(10),
        };

        let mut conn = ctx.get_connection().await?;
        let organizations = ctx.service.list(&mut conn, &pagination).await?;
        let total = organizations.len() as i64;

        Ok(HttpResponse::Ok().json(
            ApiResponseBuilder::success()
                .with_message("Organizations retrieved successfully")
                .with_data(PaginatedResponse {
                    data: organizations,
                    meta: PaginationMeta {
                        current_page: pagination.page,
                        per_page: pagination.per_page,
                        total_items: total,
                        total_pages: (total as f64 / pagination.per_page as f64).ceil() as i64,
                        has_next_page: pagination.page * pagination.per_page < total,
                        has_previous_page: pagination.page > 1,
                    },
                })
                .build()
        ))
    }
}

pub mod create {
    use super::*;

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

        input.validate()?;

        let mut conn = ctx.get_connection().await?;
        let organization = ctx.service.create(&mut conn, input).await?;

        Ok(HttpResponse::Created().json(
            ApiResponseBuilder::success()
                .with_message("Organization created successfully")
                .with_data(OrganizationResponse {
                    organization: organization.into(),
                })
                .build()
        ))
    }
}

pub mod update {
    use super::*;

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

        input.validate()?;

        let mut conn = ctx.get_connection().await?;
        let organization = ctx.service.update(&mut conn, org_id, input).await?;

        Ok(HttpResponse::Ok().json(
            ApiResponseBuilder::success()
                .with_message("Organization updated successfully")
                .with_data(OrganizationResponse {
                    organization: organization.into(),
                })
                .build()
        ))
    }
}

pub mod delete {
    use super::*;

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


        let mut conn = ctx.get_connection().await?;
        ctx.service.delete(&mut conn, org_id).await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
