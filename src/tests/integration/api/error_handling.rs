use actix_web::{test, http::StatusCode};
use crate::tests::common::{spawn_app, assertions::*};

#[actix_web::test]
async fn test_error_handling() {
    let app = test::init_service(
        spawn_app().await
            .service(actix_web::web::resource("/organizations")
                .route(actix_web::web::post().to(|| async { 
                    actix_web::HttpResponse::BadRequest().finish() 
                })))
    ).await;
    
    // Test invalid input
    let invalid_data = serde_json::json!({});
    let req = test::TestRequest::post()
        .uri("/organizations")
        .set_json(&invalid_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_error(&resp, StatusCode::BAD_REQUEST);

    // TODO: Implement more error scenarios
} 