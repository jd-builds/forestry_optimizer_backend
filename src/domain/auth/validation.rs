use diesel::PgConnection;
use crate::{
    db::{
        models::auth::User,
        repositories::auth::{UserRepository, CreateUserParams},
    },
    error::{ApiError, Result, ErrorContext},
};

pub struct AuthValidator;

impl AuthValidator {
    /// Validates login credentials
    pub async fn validate_login<'a, R: UserRepository + Send + Sync>(
        conn: &'a mut PgConnection,
        repo: &'a R,
        email: &'a str,
        password: &'a str,
    ) -> Result<User> {
        let user = repo.find_by_email(conn, email)
            .await?
            .ok_or_else(|| ApiError::validation_with_context(
                "Email not found",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "email",
                    "code": "NOT_FOUND",
                    "value": email
                }))
            ))?;

        if !User::verify_password(password, &user.password)? {
            return Err(ApiError::validation_with_context(
                "Invalid password",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "password",
                    "code": "INVALID",
                }))
            ));
        }

        Ok(user)
    }

    /// Validates registration input
    pub async fn validate_registration<'a, R: UserRepository + Send + Sync>(
        conn: &'a mut PgConnection,
        repo: &'a R,
        params: &'a CreateUserParams<'a>,
    ) -> Result<()> {
        // Check if user already exists
        if repo.find_by_email(conn, params.email).await?.is_some() {
            return Err(ApiError::validation_with_context(
                "Email already in use",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "email",
                    "code": "DUPLICATE",
                    "value": params.email
                }))
            ));
        }

        // Check if phone number already in use
        if repo.find_by_phone_number(conn, params.phone_number).await?.is_some() {
            return Err(ApiError::validation_with_context(
                "Phone number already in use",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "phone_number",
                    "code": "DUPLICATE",
                    "value": params.phone_number
                }))
            ));
        }

        Ok(())
    }
} 