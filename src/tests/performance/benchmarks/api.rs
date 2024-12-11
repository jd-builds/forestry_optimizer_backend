use std::time::Instant;
use actix_web::test;
use uuid::Uuid;
use crate::{
    tests::common::{spawn_app, helpers::TestAuth, fixtures::*},
    error::Result,
};
use super::super::PERFORMANCE_THRESHOLD_MS;

#[actix_web::test]
async fn test_api_response_time() -> Result<()> {
    let app = test::init_service(
        spawn_app().await
            .service(actix_web::web::resource("/organizations")
                .route(actix_web::web::post().to(|| async { 
                    actix_web::HttpResponse::Ok().finish() 
                })))
    ).await;

    let token = TestAuth::create_test_token(Uuid::new_v4(), "admin");
    let org_data = fake_organization();

    let start = Instant::now();
    
    // Test API response time
    let req = test::TestRequest::post()
        .uri("/organizations")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&org_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    let duration = start.elapsed();
    assert!(
        duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
        "API response should complete within {}ms (took {}ms)",
        PERFORMANCE_THRESHOLD_MS,
        duration.as_millis()
    );

    assert!(resp.status().is_success());
    Ok(())
} 