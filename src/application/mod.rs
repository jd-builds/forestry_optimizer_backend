//! Application layer implementation
//! 
//! This module contains the application-specific logic, including
//! HTTP handlers, middleware, and service implementations.

pub mod api;
pub mod config;
pub mod services;
pub mod server;

use std::sync::Arc;
use crate::domain::services::{AuthService, OrganizationService};

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<dyn AuthService>,
    pub organization_service: Arc<dyn OrganizationService>,
}

impl AppState {
    pub fn new(
        auth_service: Arc<dyn AuthService>,
        organization_service: Option<Arc<dyn OrganizationService>>,
    ) -> Self {
        Self {
            auth_service,
            organization_service: organization_service.expect("Organization service is required"),
        }
    }
}