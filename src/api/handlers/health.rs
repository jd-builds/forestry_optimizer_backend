//! Health check handlers
//! 
//! This module provides handler functions for health monitoring endpoints.
//! It includes system metrics, database connectivity checks, and resource
//! utilization monitoring to ensure the service is operating optimally.

use actix_web::{web, HttpResponse};
use crate::{
    api::dto::{health::{HealthStatus, SystemMetrics}, responses::ApiResponseBuilder, ErrorResponse},
    database::DbPool, error::Result
};
use diesel::prelude::*;
use sysinfo::{System, SystemExt, CpuExt};
use tracing::{info, error};

/// Quick health check that verifies the service process is running
/// and has sufficient resources. Returns DOWN if memory usage
/// exceeds 95%.
/// 
/// This endpoint is designed to be:
/// - Fast: <10ms response time
/// - Lightweight: Minimal resource usage
/// - Independent: No external dependency checks
/// 
/// # Returns
/// Returns a 200 OK response with basic health status
#[utoipa::path(
    get,
    path = "/v1/health/live",
    responses(
        (status = 200, description = "Service is alive", body = HealthStatus),
        (status = 503, description = "Service is not alive", body = ErrorResponse)
    ),
    tag = "health"
)]
pub async fn liveness() -> Result<HttpResponse> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let memory_usage = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
    let status = if memory_usage < 95.0 { "UP" } else { "DOWN" };

    Ok(HttpResponse::Ok().json(
        ApiResponseBuilder::success()
            .with_message("Service is live")
            .with_data(serde_json::json!({
                "status": status,
                "memory_usage_percentage": memory_usage,
            }))
            .build()
    ))
}

/// Deep health check that verifies all service dependencies are
/// functioning correctly. This includes:
/// - Database connectivity
/// - Connection pool health
/// - Resource availability
/// 
/// Status codes:
/// - 200: Service is fully operational
/// - 429: Service is degraded (high connection pool usage)
/// - 503: Service is unavailable (database unreachable)
/// 
/// # Arguments
/// * `pool` - Database connection pool to check
/// 
/// # Returns
/// Returns appropriate HTTP status code based on service health
#[utoipa::path(
    get,
    path = "/v1/health/ready",
    responses(
        (status = 200, description = "Service is ready", body = HealthStatus),
        (status = 503, description = "Service is not ready", body = ErrorResponse)
    ),
    tag = "health"
)]
pub async fn readiness(pool: web::Data<DbPool>) -> Result<HttpResponse> {
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

    let pool_state = pool.state();
    let total_connections = pool_state.connections + pool_state.idle_connections;
    let pool_status = if pool_state.connections as f32 / total_connections as f32 > 0.9 {
        "DEGRADED"
    } else if db_status { "UP" } else { "DOWN" };

    let status_code = match pool_status {
        "UP" => 200,
        "DEGRADED" => 429,
        _ => 503,
    };

    Ok(HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
        .json(ApiResponseBuilder::success()
            .with_message("Readiness check")
            .with_data(serde_json::json!({
                "status": pool_status,
                "checks": {
                    "database": {
                        "status": if db_status { "UP" } else { "DOWN" },
                        "pool": {
                            "active_connections": pool_state.connections,
                            "idle_connections": pool_state.idle_connections,
                            "total_connections": total_connections,
                            "usage_percentage": (pool_state.connections as f32 / total_connections as f32) * 100.0
                        }
                    }
                }
            }))
            .build()
        ))
}

/// Comprehensive health check endpoint that provides detailed system metrics
/// and health status. It includes:
/// - CPU and memory utilization
/// - Database connection pool statistics
/// - Service version information
/// 
/// # Arguments
/// * `pool` - Database connection pool to check
/// 
/// # Returns
/// Returns a 200 OK response with detailed health information
#[utoipa::path(
    get,
    path = "/v1/health",
    responses(
        (status = 200, description = "Service health status", body = HealthStatus),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "health"
)]
pub async fn health_check(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let metrics = SystemMetrics {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        memory_used: sys.used_memory(),
        memory_total: sys.total_memory(),
        memory_usage_percentage: (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0,
        db_active_connections: pool.state().connections,
        db_max_connections: pool.state().connections + pool.state().idle_connections,
    };

    Ok(HttpResponse::Ok().json(
        ApiResponseBuilder::success()
            .with_message("Service is healthy")
            .with_data(HealthStatus {
                status: "UP".to_string(),
                database: true,
                version: env!("CARGO_PKG_VERSION").to_string(),
                metrics: Some(metrics),
            })
            .build()
    ))
} 