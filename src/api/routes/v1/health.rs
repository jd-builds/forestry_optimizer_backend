use actix_web::{web, HttpResponse, Scope};
use serde::Serialize;
use crate::{
    db::DbPool,
    api::types::responses::ApiResponseBuilder,
};
use diesel::prelude::*;
use tracing::{info, error};

#[derive(Serialize)]
struct HealthStatus {
    status: String,
    database: bool,
    version: String,
}

pub fn routes() -> Scope {
    web::scope("/health")
        .route("", web::get().to(health_check))
        .route("/live", web::get().to(liveness))
        .route("/ready", web::get().to(readiness))
}

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