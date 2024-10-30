use crate::api::types::organization::{
    CreateOrganizationInput, ListOrganizationsQuery, OrganizationResponse, UpdateOrganizationInput,
};
use crate::db::{get_connection, repositories::organization, DbPool};
use crate::db::{
    models::Organization, repositories::base::PaginatedResponse, BaseRepository, PaginationParams,
};
use crate::errors::AppResult;
use actix_web::{web, HttpResponse};
use log::{debug, info};
use organization::OrganizationRepository;
use uuid::Uuid;

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
    ) -> AppResult<HttpResponse> {
        debug!(
            "Attempting to retrieve organization with id: {}",
            organization_id
        );

        let mut conn = get_connection(&pool)?;
        let org_id = *organization_id;

        let organization = OrganizationRepository::find_by_id(&mut conn, org_id)?;

        info!("Retrieved organization: {}", organization.id);
        Ok(HttpResponse::Ok().json(OrganizationResponse { organization }))
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
    ) -> AppResult<HttpResponse> {
        let limit = query.limit.unwrap_or(10);
        let offset = query.offset.unwrap_or(0);
        let page = (offset / limit) + 1;

        let mut conn = get_connection(&pool)?;

        let pagination = PaginationParams {
            page,
            per_page: limit,
        };

        let organizations = OrganizationRepository::list(&mut conn, &pagination)?;
        let total = organizations.len() as i64;

        info!("Retrieved {} organizations", organizations.len());
        Ok(HttpResponse::Ok().json(PaginatedResponse::new(organizations, total, &pagination)))
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
            (status = 500, description = "Internal server error")
        )
    )]
    pub async fn create_organization(
        pool: web::Data<DbPool>,
        new_organization: web::Json<CreateOrganizationInput>,
    ) -> AppResult<HttpResponse> {
        debug!(
            "Attempting to create new organization: {}",
            new_organization.name
        );

        let mut conn = get_connection(&pool)?;

        let organization =
            OrganizationRepository::create(&mut conn, &new_organization.into_inner().into())?;

        info!("Created new organization: {}", organization.id);
        Ok(HttpResponse::Created().json(OrganizationResponse { organization }))
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
    ) -> AppResult<HttpResponse> {
        debug!(
            "Attempting to update organization with id: {}",
            organization_id
        );

        let mut conn = get_connection(&pool)?;
        let org_id = *organization_id;

        // First get the existing organization
        let mut organization = OrganizationRepository::find_by_id(&mut conn, org_id)?;

        // Update only the necessary fields
        organization.name = updated_organization.name.clone();
        organization.updated_at = chrono::Utc::now();

        // Perform the update
        let organization = OrganizationRepository::update(&mut conn, org_id, &organization)?;

        info!("Updated organization: {}", organization.id);
        Ok(HttpResponse::Ok().json(OrganizationResponse { organization }))
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
    ) -> AppResult<HttpResponse> {
        debug!(
            "Attempting to delete organization with id: {}",
            organization_id
        );

        let mut conn = get_connection(&pool)?;
        let org_id = *organization_id;

        OrganizationRepository::soft_delete(&mut conn, org_id)?;

        info!("Deleted organization: {}", org_id);
        Ok(HttpResponse::NoContent().finish())
    }
}
