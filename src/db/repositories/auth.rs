//! Authentication repository implementations
//! 
//! This module provides concrete implementations of the authentication
//! repository traits for users and various types of tokens.

use crate::{
    api::utils::PaginationParams,
    db::{
        models::auth::{User, RefreshToken, PasswordResetToken, EmailVerificationToken, Role},
        schema::{users, refresh_tokens},
        repositories::Repository,
    },
    error::{Result, ApiError, ErrorCode},
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use tracing::{error, warn, info};
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateUserParams<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone_number: &'a str,
    pub password: &'a str,
    pub org_id: Uuid,
}

/// User-specific repository operations
#[async_trait]
pub trait UserRepository: Repository<User> {
    /// Find a user by email
    async fn find_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<User>>;
    
    /// Find a user by phone number
    async fn find_by_phone_number(&self, conn: &mut PgConnection, phone_number: &str) -> Result<Option<User>>;
    
    /// Find users by role
    async fn find_by_role(&self, conn: &mut PgConnection, role: Role) -> Result<Vec<User>>;

    /// Create a new user with a hashed password
    async fn create_with_password(
        &self,
        conn: &mut PgConnection,
        params: CreateUserParams<'_>,
    ) -> Result<User>;
}

/// Concrete implementation of the user repository
pub struct UserRepositoryImpl;

#[async_trait]
impl Repository<User> for UserRepositoryImpl {
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<User> {
        users::table
            .filter(users::id.eq(id))
            .filter(users::deleted_at.is_null())
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    warn!(
                        error_code = %ErrorCode::NotFound,
                        user_id = %id,
                        "User not found"
                    );
                    ApiError::not_found(format!("User with id {} not found", id))
                }
                _ => {
                    error!(
                        error_code = %ErrorCode::DatabaseError,
                        user_id = %id,
                        error = %e,
                        "Database error occurred while finding user"
                    );
                    ApiError::database_error("Failed to find user", Some(serde_json::json!({
                        "error": e.to_string()
                    })))
                }
            })
    }

    async fn create(&self, conn: &mut PgConnection, model: &User) -> Result<User> {
        diesel::insert_into(users::table)
            .values(model)
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to create user: {}", e);
                ApiError::database_error(
                    "Failed to create user",
                    Some(serde_json::json!({
                        "error": e.to_string(),
                        "details": format!("{:?}", e)
                    }))
                )
            })
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, model: &User) -> Result<User> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(model)
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to update user: {}", e);
                ApiError::database_error("Failed to update user", None)
            })
    }

    async fn soft_delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<User> {
        let now = Utc::now();
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(users::deleted_at.eq(Some(now)))
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to soft delete user: {}", e);
                ApiError::database_error("Failed to soft delete user", None)
            })
    }

    async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<User>> {
        let query = users::table
            .filter(users::deleted_at.is_null())
            .order_by((users::created_at.desc(), users::id.desc()))
            .offset(pagination.get_offset())
            .limit(pagination.get_limit());

        info!("SQL Query: {}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

        query.load(conn)
            .map_err(|e| {
                error!("Failed to list users: {}", e);
                ApiError::database_error(
                    "Failed to list users",
                    Some(serde_json::json!({
                        "error": e.to_string(),
                        "details": format!("{:?}", e)
                    }))
                )
            })
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<User>> {
        users::table
            .filter(users::email.eq(email))
            .filter(users::deleted_at.is_null())
            .select(User::as_select())
            .first(conn)
            .optional()
            .map_err(|e| {
                error!("Failed to find user by email: {}", e);
                ApiError::database_error("Failed to find user by email", None)
            })
    }

    async fn find_by_phone_number(&self, conn: &mut PgConnection, phone_number: &str) -> Result<Option<User>> {
        users::table
            .filter(users::phone_number.eq(phone_number))
            .filter(users::deleted_at.is_null())
            .select(User::as_select())
            .first(conn)
            .optional()
            .map_err(|e| {
                error!("Failed to find user by phone number: {}", e);
                ApiError::database_error("Failed to find user by phone number", None)
            })
    }

    async fn find_by_role(&self, conn: &mut PgConnection, role: Role) -> Result<Vec<User>> {
        users::table
            .filter(users::role.eq(role))
            .filter(users::deleted_at.is_null())
            .select(User::as_select())
            .load(conn)
            .map_err(|e| {
                error!("Failed to find users by role: {}", e);
                ApiError::database_error("Failed to find users by role", None)
            })
    }

    async fn create_with_password(
        &self,
        conn: &mut PgConnection,
        params: CreateUserParams<'_>,
    ) -> Result<User> {
        let hashed_password = User::hash_password(params.password)?;
        let now = Utc::now();

        let user = User {
            id: Uuid::new_v4(),
            first_name: params.first_name.to_string(),
            last_name: params.last_name.to_string(),
            email: params.email.to_string(),
            phone_number: params.phone_number.to_string(),
            password: hashed_password,
            is_supervisor: false,
            org_id: params.org_id,
            role: Role::Admin,
            email_verified: false,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        self.create(conn, &user).await
    }
}

/// Refresh token repository operations
#[async_trait]
pub trait RefreshTokenRepository: Repository<RefreshToken> {
    /// Create a new refresh token for a user
    async fn create_for_user(&self, conn: &mut PgConnection, user_id: Uuid) -> Result<RefreshToken>;
    
    /// Find a refresh token by its token string
    async fn find_by_token(&self, conn: &mut PgConnection, token: &str) -> Result<Option<RefreshToken>>;
    
    /// Revoke all refresh tokens for a user
    async fn revoke_all_for_user(&self, conn: &mut PgConnection, user_id: Uuid) -> Result<()>;
}

/// Concrete implementation of the refresh token repository
pub struct RefreshTokenRepositoryImpl;

#[async_trait]
impl Repository<RefreshToken> for RefreshTokenRepositoryImpl {
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<RefreshToken> {
        refresh_tokens::table
            .filter(refresh_tokens::id.eq(id))
            .filter(refresh_tokens::deleted_at.is_null())
            .select(RefreshToken::as_select())
            .first(conn)
            .map_err(|e| {
                error!("Failed to find refresh token: {}", e);
                ApiError::not_found(format!("Refresh token with id {} not found", id))
            })
    }

    async fn create(&self, conn: &mut PgConnection, model: &RefreshToken) -> Result<RefreshToken> {
        diesel::insert_into(refresh_tokens::table)
            .values(model)
            .returning(RefreshToken::as_select())
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to create refresh token: {}", e);
                ApiError::database_error("Failed to create refresh token", None)
            })
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, model: &RefreshToken) -> Result<RefreshToken> {
        diesel::update(refresh_tokens::table)
            .filter(refresh_tokens::id.eq(id))
            .set(model)
            .returning(RefreshToken::as_select())
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to update refresh token: {}", e);
                ApiError::database_error("Failed to update refresh token", None)
            })
    }

    async fn soft_delete(&self, conn: &mut PgConnection, id: Uuid) -> Result<RefreshToken> {
        let now = Utc::now();
        diesel::update(refresh_tokens::table)
            .filter(refresh_tokens::id.eq(id))
            .set(refresh_tokens::deleted_at.eq(Some(now)))
            .returning(RefreshToken::as_select())
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to soft delete refresh token: {}", e);
                ApiError::database_error("Failed to soft delete refresh token", None)
            })
    }

    async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<RefreshToken>> {
        refresh_tokens::table
            .filter(refresh_tokens::deleted_at.is_null())
            .offset(pagination.get_offset())
            .limit(pagination.get_limit())
            .select(RefreshToken::as_select())
            .load(conn)
            .map_err(|e| {
                error!("Failed to list refresh tokens: {}", e);
                ApiError::database_error("Failed to list refresh tokens", None)
            })
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenRepositoryImpl {
    async fn create_for_user(&self, conn: &mut PgConnection, user_id: Uuid) -> Result<RefreshToken> {
        let now = Utc::now();
        let token = Uuid::new_v4().to_string();

        let refresh_token = RefreshToken {
            id: Uuid::new_v4(),
            token,
            user_id,
            expires_at: now + Duration::days(7),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        self.create(conn, &refresh_token).await
    }

    async fn find_by_token(&self, conn: &mut PgConnection, token: &str) -> Result<Option<RefreshToken>> {
        refresh_tokens::table
            .filter(refresh_tokens::token.eq(token))
            .filter(refresh_tokens::deleted_at.is_null())
            .first::<RefreshToken>(conn)
            .optional()
            .map_err(|e| {
                error!("Failed to find refresh token: {}", e);
                ApiError::database_error("Failed to find refresh token by token", None)
            })
    }

    async fn revoke_all_for_user(&self, conn: &mut PgConnection, user_id: Uuid) -> Result<()> {
        diesel::update(refresh_tokens::table)
            .filter(refresh_tokens::user_id.eq(user_id))
            .filter(refresh_tokens::deleted_at.is_null())
            .set(refresh_tokens::deleted_at.eq(Some(Utc::now())))
            .execute(conn)
            .map_err(|e| {
                error!("Failed to revoke refresh tokens: {}", e);
                ApiError::database_error("Failed to revoke refresh tokens", None)
            })?;
        Ok(())
    }
}

/// TODO: Implement
/// Password reset token repository operations
#[allow(unused)]
#[async_trait]
pub trait PasswordResetTokenRepository: Repository<PasswordResetToken> {
    /// Create a new password reset token for a user
    async fn create_for_user(&self, conn: &mut PgConnection, user_id: Uuid) -> Result<PasswordResetToken>;
    
    /// Find a password reset token by its token string
    async fn find_by_token(&self, conn: &mut PgConnection, token: &str) -> Result<Option<PasswordResetToken>>;
}

/// TODO: Implement
/// Email verification token repository operations
#[allow(unused)]
#[async_trait]
pub trait EmailVerificationTokenRepository: Repository<EmailVerificationToken> {
    /// Create a new email verification token for a user
    async fn create_for_user(&self, conn: &mut PgConnection, user_id: Uuid) -> Result<EmailVerificationToken>;
    
    /// Find an email verification token by its token string
    async fn find_by_token(&self, conn: &mut PgConnection, token: &str) -> Result<Option<EmailVerificationToken>>;
} 