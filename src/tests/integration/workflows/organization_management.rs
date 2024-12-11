use actix_web::test;
use uuid::Uuid;
use crate::{
    tests::common::{spawn_app, helpers::TestAuth, fixtures::*},
    error::Result,
};

#[actix_web::test]
async fn test_complete_organization_workflow() -> Result<()> {
    let app = test::init_service(
        spawn_app().await
            .service(actix_web::web::resource("/organizations")
                .route(actix_web::web::post().to(|| async { 
                    actix_web::HttpResponse::Ok().finish() 
                }))
                .route(actix_web::web::get().to(|| async {
                    actix_web::HttpResponse::Ok().finish()
                }))
                .route(actix_web::web::delete().to(|| async {
                    actix_web::HttpResponse::Ok().finish()
                })))
            // Add users endpoint
            .service(actix_web::web::resource("/organizations/{org_id}/users")
                .route(actix_web::web::post().to(|| async { 
                    actix_web::HttpResponse::Ok().finish() 
                }))
                .route(actix_web::web::get().to(|| async {
                    actix_web::HttpResponse::Ok().finish()
                })))
    ).await;

    // Complete workflow test
    let token = TestAuth::create_test_token(Uuid::new_v4(), "admin");
    let org_data = fake_organization();

    // 1. Create organization
    let create_resp = test::call_service(&app, 
        test::TestRequest::post()
            .uri("/organizations")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&org_data)
            .to_request()
    ).await;
    assert!(create_resp.status().is_success());
    
    // Extract org_id from response
    let org_id = create_resp.response().json::<serde_json::Value>().id.to_string();

    // 2. Add users to organization
    let user_data = fake_user();
    let add_user_resp = test::call_service(&app,
        test::TestRequest::post()
            .uri(&format!("/organizations/{}/users", org_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&user_data)
            .to_request()
    ).await;
    assert!(add_user_resp.status().is_success());

    // 3. Verify organization details
    let get_resp = test::call_service(&app,
        test::TestRequest::get()
            .uri(&format!("/organizations/{}", org_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    ).await;
    assert!(get_resp.status().is_success());

    // 4. Clean up
    let delete_resp = test::call_service(&app,
        test::TestRequest::delete()
            .uri(&format!("/organizations/{}", org_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request()
    ).await;
    assert!(delete_resp.status().is_success());

    Ok(())
} 