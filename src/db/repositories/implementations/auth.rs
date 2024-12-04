//! Authentication repository implementations
//! 
//! This module provides concrete implementations of the authentication
//! repository traits for users and various types of tokens.

use crate::{
    api::types::pagination::PaginationParams,
    db::{
        models::auth::{User, RefreshToken},
        schema::{users, refresh_tokens},
        repositories::traits::{
            Repository,
            auth::{
                UserRepository as UserRepositoryTrait,
                RefreshTokenRepository as RefreshTokenRepositoryTrait,
                CreateUserParams,
            },
        },
    },
    error::{Result, ApiError, ErrorCode, ErrorContext},
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use tracing::error;
use uuid::Uuid;

/// Concrete implementation of the user repository
pub struct UserRepositoryImpl;

#[async_trait]
impl Repository<User> for UserRepositoryImpl {
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<User> {
        users::table
            .filter(users::id.eq(id))
            .filter(users::deleted_at.is_null())
            .first(conn)
            .map_err(|e| {
                error!("Failed to find user by id: {}", e);
                ApiError::new(ErrorCode::NotFound, "User not found", ErrorContext::default())
            })
    }

    async fn create(&self, conn: &mut PgConnection, model: &User) -> Result<User> {
        diesel::insert_into(users::table)
            .values(model)
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to create user: {}", e);
                ApiError::new(ErrorCode::DatabaseError, "Failed to create user", ErrorContext::default())
            })
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, model: &User) -> Result<User> {
        diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(model)
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to update user: {}", e);
                ApiError::new(ErrorCode::DatabaseError, "Failed to update user", ErrorContext::default())
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
                ApiError::new(ErrorCode::DatabaseError, "Failed to soft delete user", ErrorContext::default())
            })
    }

    async fn list(&self, conn: &mut PgConnection, pagination: &PaginationParams) -> Result<Vec<User>> {
        users::table
            .filter(users::deleted_at.is_null())
            .offset(pagination.get_offset())
            .limit(pagination.get_limit())
            .load(conn)
            .map_err(|e| {
                error!("Failed to list users: {}", e);
                ApiError::new(ErrorCode::DatabaseError, "Failed to list users", ErrorContext::default())
            })
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepositoryImpl {
    async fn find_by_email(&self, conn: &mut PgConnection, email: &str) -> Result<Option<User>> {
        users::table
            .filter(users::email.eq(email))
            .filter(users::deleted_at.is_null())
            .select(User::as_select())
            .first(conn)
            .optional()
            .map_err(|e| {
                error!("Failed to find user by email: {}", e);
                ApiError::new(ErrorCode::DatabaseError, "Failed to find user", ErrorContext::default())
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
                ApiError::new(ErrorCode::DatabaseError, "Failed to find user", ErrorContext::default())
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
            role: crate::db::models::auth::Role::Admin,
            email_verified: false,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        self.create(conn, &user).await
    }
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
                ApiError::new(ErrorCode::NotFound, "Refresh token not found", ErrorContext::default())
            })
    }

    async fn create(&self, conn: &mut PgConnection, model: &RefreshToken) -> Result<RefreshToken> {
        diesel::insert_into(refresh_tokens::table)
            .values(model)
            .returning(RefreshToken::as_select())
            .get_result(conn)
            .map_err(|e| {
                error!("Failed to create refresh token: {}", e);
                ApiError::new(ErrorCode::DatabaseError, "Failed to create refresh token", ErrorContext::default())
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
                ApiError::new(ErrorCode::DatabaseError, "Failed to update refresh token", ErrorContext::default())
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
                ApiError::new(ErrorCode::DatabaseError, "Failed to soft delete refresh token", ErrorContext::default())
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
                ApiError::new(ErrorCode::DatabaseError, "Failed to list refresh tokens", ErrorContext::default())
            })
    }
}

#[async_trait]
impl RefreshTokenRepositoryTrait for RefreshTokenRepositoryImpl {
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
                ApiError::new(ErrorCode::DatabaseError, "Failed to find refresh token", ErrorContext::default())
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
                ApiError::new(ErrorCode::DatabaseError, "Failed to revoke refresh tokens", ErrorContext::default())
            })?;
        Ok(())
    }
}

// Similar implementations for PasswordResetTokenRepositoryImpl and EmailVerificationTokenRepositoryImpl... 