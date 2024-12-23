use diesel::PgConnection;
use crate::{
    db::{
        models::auth::User,
        repositories::{auth::{CreateUserParams, UserRepository}, organization::OrganizationRepositoryImpl, Repository},
    },
    error::{ApiError, ErrorContext, Result},
};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();
}

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
        // Validate organization exists
        let org_repo = OrganizationRepositoryImpl;
        if org_repo.find_by_id(conn, params.org_id).await.is_err() {
            return Err(ApiError::validation_with_context(
                "Organization not found",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "org_id",
                    "code": "NOT_FOUND",
                    "value": params.org_id
                }))
            ));
        }

        // Validate email format
        if !EMAIL_REGEX.is_match(params.email) {
            return Err(ApiError::validation_with_context(
                "Invalid email format",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "email",
                    "code": "INVALID_FORMAT",
                    "value": params.email
                }))
            ));
        }

        // Validate password length (minimum 8 characters)
        if params.password.len() < 8 {
            return Err(ApiError::validation_with_context(
                "Password too short",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "password",
                    "code": "TOO_SHORT",
                    "min_length": 8
                }))
            ));
        }

        // Validate password contains numbers
        if !params.password.chars().any(|c| c.is_numeric()) {
            return Err(ApiError::validation_with_context(
                "Password must contain at least one number",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "password",
                    "code": "MISSING_NUMBER"
                }))
            ));
        }

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