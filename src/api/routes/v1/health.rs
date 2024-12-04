//! Health check routes
//! 
//! This module configures routes for health monitoring endpoints.

use actix_web::web;
use crate::api::handlers::health;

/// Configures and returns the health check routes
/// 
/// Sets up the following endpoints:
/// - GET /health - Comprehensive health check with system metrics
/// - GET /health/live - Quick liveness probe
/// - GET /health/ready - Deep readiness check with dependency status
pub fn routes() -> actix_web::Scope {
    web::scope("")
        .route("/live", web::get().to(health::liveness))
        .route("/ready", web::get().to(health::readiness))
        .route("/health", web::get().to(health::health_check))
} 