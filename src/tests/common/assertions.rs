use actix_web::{
    dev::ServiceResponse,
    body::MessageBody,
    http::StatusCode,
};
use serde::Serialize;
use pretty_assertions::assert_eq;
use crate::db::models::{auth::User, Organization};

pub fn assert_success<T: Serialize>(response: &ServiceResponse<impl MessageBody>, _expected_data: &T) {
    assert_eq!(response.status(), StatusCode::OK);
    // TODO: Add more specific assertions
}

pub fn assert_error(response: &ServiceResponse<impl MessageBody>, expected_status: StatusCode) {
    assert_eq!(response.status(), expected_status);
    // TODO: Add more specific error assertions
}

pub fn assert_user_matches(actual: &User, expected: &serde_json::Value) {
    assert_eq!(actual.first_name, expected["first_name"].as_str().unwrap());
    assert_eq!(actual.last_name, expected["last_name"].as_str().unwrap());
    assert_eq!(actual.email, expected["email"].as_str().unwrap());
    assert_eq!(actual.phone_number, expected["phone_number"].as_str().unwrap());
    assert_eq!(actual.org_id.to_string(), expected["org_id"].as_str().unwrap());
    assert_eq!(actual.is_supervisor, expected["is_supervisor"].as_bool().unwrap());
    assert_eq!(actual.email_verified, expected["email_verified"].as_bool().unwrap());
}

pub fn assert_organization_matches(actual: &Organization, expected: &serde_json::Value) {
    assert_eq!(actual.name, expected["name"].as_str().unwrap());
}

pub fn assert_user_list_ordered_by_created_at(users: &[User]) {
    for i in 1..users.len() {
        assert!(users[i-1].created_at >= users[i].created_at, 
            "Users should be ordered by created_at descending");
    }
}

pub fn assert_organization_list_ordered_by_created_at(orgs: &[Organization]) {
    for i in 1..orgs.len() {
        assert!(orgs[i-1].created_at >= orgs[i].created_at,
            "Organizations should be ordered by created_at descending");
    }
} 