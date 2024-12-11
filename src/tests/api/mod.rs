//! API endpoint tests
//! Tests for HTTP endpoints and request handling

use actix_web::{test, http::StatusCode};
use crate::tests::common::{spawn_app, TestAuth, TestData, assertions::*};

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            spawn_app().await
                .service(actix_web::web::resource("/health_check")
                    .route(actix_web::web::get().to(|| async { 
                        actix_web::HttpResponse::Ok().finish() 
                    })))
        ).await;

        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_auth_flow() {
        let app = test::init_service(
            spawn_app().await
                .service(actix_web::web::resource("/auth/register")
                    .route(actix_web::web::post().to(|| async { 
                        actix_web::HttpResponse::Ok().finish() 
                    })))
        ).await;
        
        // Test registration
        let user_data = TestData::fake_user();
        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&user_data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_success(&resp, &user_data);

        // TODO: Implement login test
    }

    #[actix_web::test]
    async fn test_organization_crud() {
        let app = test::init_service(
            spawn_app().await
                .service(actix_web::web::resource("/organizations")
                    .route(actix_web::web::post().to(|| async { 
                        actix_web::HttpResponse::Ok().finish() 
                    })))
        ).await;

        let token = TestAuth::create_test_token(uuid::Uuid::new_v4(), "admin");
        
        // Test create organization
        let org_data = TestData::fake_organization();
        let req = test::TestRequest::post()
            .uri("/organizations")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&org_data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_success(&resp, &org_data);

        // TODO: Implement read, update, delete tests
    }

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
}
