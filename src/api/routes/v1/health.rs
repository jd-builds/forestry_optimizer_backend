//! Health check endpoints
//! 
//! This module provides endpoints for monitoring the health and status
//! of the service. It includes basic health checks, liveness probes,
//! and readiness checks that verify database connectivity.

use actix_web::{web, HttpResponse, Scope};
use serde::Serialize;
use crate::{
    db::DbPool,
    api::types::responses::ApiResponseBuilder,
};
use diesel::prelude::*;
use tracing::{info, error};

/// Response structure for health check endpoints
#[derive(Serialize)]
struct HealthStatus {
    /// Current status of the service ("UP" or "DOWN")
    status: String,
    /// Whether the database connection is healthy
    database: bool,
    /// Current version of the service
    version: String,
}

/// Configures and returns the health check routes
/// 
/// Sets up the following endpoints:
/// - GET /health - Basic health check
/// - GET /health/live - Liveness probe
/// - GET /health/ready - Readiness probe (includes database check)
pub fn routes() -> Scope {
    web::scope("/health")
        .route("", web::get().to(health_check))
        .route("/live", web::get().to(liveness))
        .route("/ready", web::get().to(readiness))
}

/// Basic health check endpoint
/// 
/// Returns a simple status check indicating the service is running.
/// This endpoint should be fast and not depend on any external services.
/// 
/// # Returns
/// 
/// Returns a 200 OK response with basic health information
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(
        ApiResponseBuilder::success()
            .with_message("Service is healthy")
            .with_data(HealthStatus {
                status: "UP".to_string(),
                database: true,
                version: env!("CARGO_PKG_VERSION").to_string(),
            })
            .build()
    )
}

/// Liveness probe endpoint
/// 
/// Indicates whether the service is alive and running.
/// This endpoint should be very lightweight and only check
/// if the service process is running.
/// 
/// # Returns
/// 
/// Returns a 200 OK response if the service is alive
async fn liveness() -> HttpResponse {
    HttpResponse::Ok().json(
        ApiResponseBuilder::success()
            .with_message("Service is live")
            .with_data(serde_json::json!({
                "status": "UP"
            }))
            .build()
    )
}

/// Readiness probe endpoint
/// 
/// Checks if the service is ready to handle requests.
/// This includes verifying database connectivity and any
/// other external dependencies.
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// 
/// # Returns
/// 
/// Returns a 200 OK response if ready, 503 Service Unavailable if not
async fn readiness(pool: web::Data<DbPool>) -> HttpResponse {
    let db_status = match pool.get() {
        Ok(mut conn) => {
            match diesel::select(diesel::dsl::sql::<diesel::sql_types::Bool>("SELECT 1")).get_result::<bool>(&mut conn) {
                Ok(_) => {
                    info!("Database connection successful");
                    true
                },
                Err(e) => {
                    error!("Database query failed: {}", e);
                    false
                }
            }
        },
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            false
        }
    };

    let status = if db_status { "UP" } else { "DOWN" };
    let status_code = if db_status { 200 } else { 503 };

    HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
        .json(ApiResponseBuilder::success()
            .with_message("Readiness check")
            .with_data(serde_json::json!({
                "status": status,
                "checks": {
                    "database": {
                        "status": if db_status { "UP" } else { "DOWN" }
                    }
                }
            }))
            .build()
        )
} 