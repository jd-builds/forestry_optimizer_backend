//! Health check types
//! 
//! This module defines the types used for health check responses,
//! including system metrics and health status information.

use serde::Serialize;

/// Response structure for health check endpoints
#[derive(Serialize)]
pub struct HealthStatus {
    /// Current status of the service ("UP" or "DOWN")
    pub status: String,
    /// Whether the database connection is healthy
    pub database: bool,
    /// Current version of the service
    pub version: String,
    /// System metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<SystemMetrics>,
}

/// System metrics for detailed health information
#[derive(Serialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage in bytes
    pub memory_used: u64,
    /// Total memory in bytes
    pub memory_total: u64,
    /// Memory usage percentage
    pub memory_usage_percentage: f32,
    /// Number of active database connections
    pub db_active_connections: u32,
    /// Maximum database connections
    pub db_max_connections: u32,
} 