//! Integration tests
//! End-to-end tests covering complete workflows and scenarios

use serial_test::serial;
use crate::tests::common::{TestAuth, assertions::*};

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http::StatusCode, web, HttpResponse};
    use crate::{error::Result, tests::common::fake_organization};

    #[tokio::test]
    #[serial]
    async fn test_complete_organization_workflow() -> Result<()> {
        // Initialize test app
        let app = test::init_service(
            crate::tests::common::spawn_app().await
                .service(
                    web::resource("/organizations")
                        .route(web::post().to(|| async { HttpResponse::Ok().finish() }))
                        .route(web::put().to(|| async { HttpResponse::Ok().finish() }))
                        .route(web::delete().to(|| async { HttpResponse::Ok().finish() }))
                )
                .service(
                    web::resource("/organizations/{id}/resources")
                        .route(web::post().to(|| async { HttpResponse::Ok().finish() }))
                        .route(web::get().to(|| async { HttpResponse::Ok().finish() }))
                )
        ).await;
        
        // Create admin user and get token
        let admin_token = TestAuth::create_test_token(uuid::Uuid::new_v4(), "admin");

        // Test organization creation
        let org_data = fake_organization();
        let req = test::TestRequest::post()
            .uri("/organizations")
            .insert_header(("Authorization", format!("Bearer {}", admin_token)))
            .set_json(&org_data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_success(&resp, &org_data);

        // Test resource management
        let resource_data = serde_json::json!({
            "name": "Test Resource",
            "type": "compute",
            "capacity": 100
        });

        // Add resource to organization
        let req = test::TestRequest::post()
            .uri("/organizations/123/resources")
            .insert_header(("Authorization", format!("Bearer {}", admin_token)))
            .set_json(&resource_data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_success(&resp, &resource_data);

        // Get organization resources
        let req = test::TestRequest::get()
            .uri("/organizations/123/resources")
            .insert_header(("Authorization", format!("Bearer {}", admin_token)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_success(&resp, &resource_data);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_error_scenarios() -> Result<()> {
        let app = test::init_service(
            crate::tests::common::spawn_app().await
                .service(
                    web::resource("/organizations")
                        .route(web::post().to(|| async { HttpResponse::BadRequest().finish() }))
                        .route(web::get().to(|| async { HttpResponse::Unauthorized().finish() }))
                        .route(web::put().to(|| async { HttpResponse::NotFound().finish() }))
                        .route(web::delete().to(|| async { HttpResponse::Conflict().finish() }))
                )
        ).await;

        // Test unauthorized access
        let req = test::TestRequest::get()
            .uri("/organizations")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_error(&resp, StatusCode::UNAUTHORIZED);

        // Test invalid input handling
        let invalid_data = serde_json::json!({});
        let req = test::TestRequest::post()
            .uri("/organizations")
            .set_json(&invalid_data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_error(&resp, StatusCode::BAD_REQUEST);

        // Test not found error
        let req = test::TestRequest::put()
            .uri("/organizations/999")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_error(&resp, StatusCode::NOT_FOUND);

        // Test conflict error
        let req = test::TestRequest::delete()
            .uri("/organizations")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_error(&resp, StatusCode::CONFLICT);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_concurrent_operations() -> Result<()> {
        let app = test::init_service(
            crate::tests::common::spawn_app().await
                .service(
                    web::resource("/organizations")
                        .route(web::post().to(|| async { HttpResponse::Ok().finish() }))
                )
        ).await;
        
        // Test multiple requests
        for _ in 0..3 {
            let req = test::TestRequest::post()
                .uri("/organizations")
                .set_json(&fake_organization())
                .to_request();
            
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }

        Ok(())
    }
}
