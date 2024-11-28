//! Authentication models and implementations
//! 
//! This module defines the models and implementations for authentication-related
//! functionality including refresh tokens, password reset tokens, and email
//! verification tokens.

use super::base::Timestamps;
use crate::{
    db::schema::{refresh_tokens, password_reset_tokens, email_verification_tokens, users},
    errors::{Result, ApiError, ErrorCode, ErrorContext}
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::ToSchema;
use uuid::Uuid;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

/// User roles in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::db::schema::sql_types::UserRole"]
pub enum Role {
    Admin,
    Manager,
    Operator,
}

/// Represents a user in the system with auth-specific fields
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub password: String,
    pub is_supervisor: bool,
    pub org_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub role: Role,
    pub email_verified: bool,
}

impl User {
    /// Hash a password using Argon2
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| {
                error!("Failed to hash password: {}", e);
                ApiError::new(
                    ErrorCode::InternalError,
                    "Failed to hash password",
                    ErrorContext::default()
                )
            })
    }

    /// Verify a password against its hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| {
            error!("Failed to parse password hash: {}", e);
            ApiError::new(
                ErrorCode::InternalError,
                "Failed to verify password",
                ErrorContext::default()
            )
        })?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

/// Represents a refresh token for JWT authentication
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = refresh_tokens)]
pub struct RefreshToken {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Represents a password reset token
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = password_reset_tokens)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Represents an email verification token
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = email_verification_tokens)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Timestamps for User {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
}

impl Timestamps for RefreshToken {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
}

impl Timestamps for PasswordResetToken {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
}

impl Timestamps for EmailVerificationToken {
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
} 