use actix_web::HttpResponse;
use tracing::info;

pub async fn health_check() -> HttpResponse {
    info!("Health check requested");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
} 