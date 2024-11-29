use crate::{
    common::error::Result,
    common::pagination::PaginationParams,
    domain::{
        models::Organization,
        services::OrganizationService,
    },
    application::api::types::{
        CreateOrganizationRequest,
        UpdateOrganizationRequest,
        OrganizationResponse,
        OrganizationListResponse,
    },
};
use actix_web::{web, HttpResponse};
use std::sync::Arc;
use uuid::Uuid;

pub struct OrganizationHandler<S> {
    service: Arc<S>,
}

impl<S> OrganizationHandler<S>
where
    S: OrganizationService,
{
    pub fn new(service: Arc<S>) -> Self {
        Self { service }
    }

    pub async fn create_organization(
        &self,
        req: web::Json<CreateOrganizationRequest>,
    ) -> Result<HttpResponse> {
        let organization = self.service.create_organization(req.name.clone()).await?;
        
        Ok(HttpResponse::Created().json(OrganizationResponse {
            organization: organization.into(),
        }))
    }

    pub async fn update_organization(
        &self,
        id: web::Path<Uuid>,
        req: web::Json<UpdateOrganizationRequest>,
    ) -> Result<HttpResponse> {
        let organization = self.service
            .update_organization(*id, req.name.clone())
            .await?;
        
        Ok(HttpResponse::Ok().json(OrganizationResponse {
            organization: organization.into(),
        }))
    }

    pub async fn delete_organization(
        &self,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse> {
        self.service.delete_organization(*id).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn get_organization(
        &self,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse> {
        let organization = self.service.get_organization(*id).await?;
        
        Ok(HttpResponse::Ok().json(OrganizationResponse {
            organization: organization.into(),
        }))
    }

    pub async fn list_organizations(
        &self,
        query: web::Query<PaginationParams>,
    ) -> Result<HttpResponse> {
        let organizations = self.service
            .list_organizations(query.into_inner())
            .await?;
        
        Ok(HttpResponse::Ok().json(OrganizationListResponse {
            organizations: organizations.into_iter().map(Into::into).collect(),
        }))
    }
} 