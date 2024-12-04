//! Authentication repository traits
//! 
//! This module defines the authentication-specific repository operations
//! for users and various types of tokens.

use crate::{
    db::models::auth::{User, RefreshToken, PasswordResetToken, EmailVerificationToken},
    error::Result,
};
use async_trait::async_trait;
use diesel::PgConnection;
use uuid::Uuid;

use super::base::Repository;

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
    
    /// Create a new user with a hashed password
    async fn create_with_password(
        &self,
        conn: &mut PgConnection,
        params: CreateUserParams<'_>,
    ) -> Result<User>;
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