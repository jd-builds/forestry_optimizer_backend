use fake::{Fake, Faker};
use uuid::Uuid;
use diesel::PgConnection;
use crate::{
    db::{
        models::auth::{User, Role},
        repositories::{auth::UserRepositoryImpl, Repository},
    },
    error::Result,
};
use super::constants::*;

pub fn fake_user() -> serde_json::Value {
    let uuid = Uuid::new_v4();
    serde_json::json!({
        "first_name": format!("{} {}", TEST_USER_NAME_PREFIX, Faker.fake::<String>()),
        "last_name": Faker.fake::<String>(),
        "email": format!("test{}{}", uuid, TEST_EMAIL_DOMAIN),
        "phone_number": format!("{}{}", TEST_PHONE_PREFIX, uuid.simple()),
        "password": TEST_PASSWORD,
        "role": TEST_ROLES[0],
        "org_id": Uuid::new_v4(),
        "email_verified": false
    })
}

pub async fn create_test_user(conn: &mut PgConnection, org_id: Uuid) -> Result<User> {
    let repo = UserRepositoryImpl;
    let mut user_data = fake_user();
    user_data["org_id"] = serde_json::json!(org_id);
    
    let password = User::hash_password(TEST_PASSWORD)?;
    let user = User {
        id: Uuid::new_v4(),
        first_name: user_data["first_name"].as_str().unwrap().to_string(),
        last_name: user_data["last_name"].as_str().unwrap().to_string(),
        email: user_data["email"].as_str().unwrap().to_string(),
        phone_number: user_data["phone_number"].as_str().unwrap().to_string(),
        password,
        org_id,
        role: Role::Operator,
        email_verified: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        deleted_at: None,
    };
    
    repo.create(conn, &user).await
}

pub async fn create_test_users(conn: &mut PgConnection, org_id: Uuid, count: i32) -> Result<Vec<User>> {
    let mut users = Vec::new();
    for _ in 0..count {
        users.push(create_test_user(conn, org_id).await?);
    }
    Ok(users)
} 