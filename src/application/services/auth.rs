use crate::{
    common::error::{ApiError, ErrorCode, ErrorContext, Result},
    domain::{
        models::user::{Role, User},
        repositories::UserRepository,
        services::AuthService,
    },
    infrastructure::security::{JwtManager, PasswordHasher},
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

pub struct AuthServiceImpl<R> {
    repository: Arc<R>,
    jwt_manager: Arc<JwtManager>,
    password_hasher: Arc<PasswordHasher>,
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl<R> AuthServiceImpl<R>
where
    R: UserRepository,
{
    pub fn new(
        repository: Arc<R>,
        jwt_manager: Arc<JwtManager>,
        password_hasher: Arc<PasswordHasher>,
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    ) -> Self {
        Self {
            repository,
            jwt_manager,
            password_hasher,
            pool,
        }
    }
}

#[async_trait]
impl<R> AuthService for AuthServiceImpl<R>
where
    R: UserRepository + Send + Sync,
{
    async fn register_user(
        &self,
        first_name: String,
        last_name: String,
        email: String,
        phone_number: String,
        password: String,
        org_id: Uuid,
        role: Role,
    ) -> Result<User> {
        // Check if user already exists
        if let Some(_) = self.repository.find_by_email(&mut self.pool.get().unwrap(), &email).await? {
            return Err(ApiError::new(
                ErrorCode::Conflict,
                "User with this email already exists",
                ErrorContext::new().with_details(serde_json::json!({
                    "field": "email",
                    "code": "REQUIRED",
                    "value": email
                }))
            ));
        }

        // Hash password
        let password_hash = self.password_hasher.hash_password(&password)?;

        // Create user
        let user = User {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            phone_number,
            password: password_hash,
            is_supervisor: false,
            org_id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            role,
            email_verified: false,
        };
        let created = self.repository.create(&user).await?;

        info!(
            user_id = %created.id,
            user_email = %created.email,
            user_role = %created.role,
            "Created new user"
        );

        Ok(created)
    }

    async fn login(&self, email: String, password: String) -> Result<String> {
        // Find user
        let user = self.repository.find_by_email(&mut self.pool.get().unwrap(), &email).await?
            .ok_or_else(|| ApiError::new(
                ErrorCode::Unauthorized,
                "Invalid email or password",
                ErrorContext::new()
            ))?;

        // Verify password
        if !self.password_hasher.verify_password(&password, &user.password)? {
            return Err(ApiError::new(
                ErrorCode::Unauthorized,
                "Invalid email or password",
                ErrorContext::new()
            ));
        }

        // Generate token
        let token = self.jwt_manager.create_token(
            user.id,
            user.org_id,
            user.role.to_string(),
            3600, // 1 hour
        )?;

        info!(
            user_id = %user.id,
            user_email = %user.email,
            "User logged in successfully"
        );

        Ok(token)
    }

    async fn verify_token(&self, token: &str) -> Result<User> {
        // Verify token
        let claims = self.jwt_manager.verify_token(token)?;

        // Get user
        let user = self.repository.find_by_id(claims.sub).await?;

        Ok(user)
    }

    async fn get_user(&self, id: Uuid) -> Result<User> {
        self.repository.find_by_id(id).await
    }

    async fn update_password(&self, id: Uuid, old_password: String, new_password: String) -> Result<()> {
        // Get user
        let user = self.repository.find_by_id(id).await?;

        // Verify old password
        if !self.password_hasher.verify_password(&old_password, &user.password)? {
            return Err(ApiError::new(
                ErrorCode::Unauthorized,
                "Invalid current password",
                ErrorContext::new()
            ));
        }

        // Hash new password
        let password_hash = self.password_hasher.hash_password(&new_password)?;

        // Update password
        self.repository.update_password(id, password_hash).await?;

        info!(
            user_id = %id,
            "User password updated successfully"
        );

        Ok(())
    }

    async fn update_role(&self, id: Uuid, new_role: Role) -> Result<User> {
        let updated = self.repository.update_role(id, new_role).await?;

        info!(
            user_id = %updated.id,
            user_email = %updated.email,
            new_role = %updated.role.to_string(),
            "User role updated"
        );

        Ok(updated)
    }
} 