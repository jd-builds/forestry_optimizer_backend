//! Authentication service implementation
//! 
//! This module provides JWT token management and authentication services.
//! It handles token generation, validation, and refresh operations.

use crate::{
    db::{
        models::auth::{User, RefreshToken},
        repositories::implementations::auth::{UserRepositoryImpl, RefreshTokenRepositoryImpl},
        repositories::traits::{
            Repository,
            auth::{UserRepository, RefreshTokenRepository},
        },
        DbPool,
    },
    errors::{Result, ApiError, ErrorCode, ErrorContext},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation,
    errors::ErrorKind as JwtErrorKind,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

const JWT_SECRET: &[u8] = b"your-secret-key"; // TODO: Move to config
const JWT_EXPIRATION: i64 = 60 * 60; // 1 hour in seconds

/// Claims stored in JWT tokens
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Organization ID
    pub org_id: String,
    /// User role
    pub role: String,
    /// Issued at
    pub iat: i64,
    /// Expiration time
    pub exp: i64,
}

/// Authentication service for JWT operations
pub struct AuthService;

impl AuthService {
    /// Generate a new JWT token for a user
    pub fn generate_token(user: &User) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(JWT_EXPIRATION);

        let claims = Claims {
            sub: user.id.to_string(),
            org_id: user.org_id.to_string(),
            role: format!("{:?}", user.role).to_uppercase(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET),
        )
        .map_err(|e| {
            error!("Failed to generate JWT token: {}", e);
            ApiError::new(
                ErrorCode::InternalError,
                "Failed to generate token",
                ErrorContext::default()
            )
        })
    }

    /// Validate a JWT token and return the claims
    pub fn validate_token(token: &str) -> Result<Claims> {
        let validation = Validation::default();

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET),
            &validation,
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => {
                match e.kind() {
                    JwtErrorKind::ExpiredSignature => {
                        Err(ApiError::new(
                            ErrorCode::Unauthorized,
                            "Token expired",
                            ErrorContext::default()
                        ))
                    }
                    JwtErrorKind::InvalidToken => {
                        Err(ApiError::new(
                            ErrorCode::Unauthorized,
                            "Invalid token",
                            ErrorContext::default()
                        ))
                    }
                    _ => {
                        error!("Failed to validate JWT token: {}", e);
                        Err(ApiError::new(
                            ErrorCode::InternalError,
                            "Token validation failed",
                            ErrorContext::default()
                        ))
                    }
                }
            }
        }
    }

    /// Refresh an access token using a refresh token
    pub async fn refresh_token(pool: &DbPool, refresh_token: &str) -> Result<(String, RefreshToken)> {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            ApiError::new(
                ErrorCode::DatabaseError,
                "Database connection error",
                ErrorContext::default()
            )
        })?;

        let repo = RefreshTokenRepositoryImpl;
        // Find the refresh token
        let token = repo.find_by_token(&mut conn, refresh_token)
            .await?
            .ok_or_else(|| ApiError::new(
                ErrorCode::Unauthorized,
                "Invalid refresh token",
                ErrorContext::default()
            ))?;

        // Check if token is expired
        if token.expires_at < Utc::now() {
            return Err(ApiError::new(
                ErrorCode::Unauthorized,
                "Refresh token expired",
                ErrorContext::default()
            ));
        }

        let user_repo = UserRepositoryImpl;
        // Find the user
        let user = user_repo.find_by_id(&mut conn, token.user_id).await?;

        // Generate new access token
        let access_token = Self::generate_token(&user)?;

        // Create new refresh token
        let new_refresh_token = repo.create_for_user(&mut conn, user.id).await?;

        // Revoke old refresh token
        repo.revoke_all_for_user(&mut conn, user.id).await?;

        Ok((access_token, new_refresh_token))
    }

    /// Login a user and generate tokens
    pub async fn login(
        pool: &DbPool,
        email: &str,
        password: &str,
    ) -> Result<(String, RefreshToken, User)> {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            ApiError::new(
                ErrorCode::DatabaseError,
                "Database connection error",
                ErrorContext::default()
            )
        })?;

        let user_repo = UserRepositoryImpl;
        let refresh_repo = RefreshTokenRepositoryImpl;

        // Find user by email
        let user = user_repo.find_by_email(&mut conn, email)
            .await?
            .ok_or_else(|| ApiError::new(
                ErrorCode::Unauthorized,
                "Invalid credentials",
                ErrorContext::default()
            ))?;

        // Verify password
        if !User::verify_password(password, &user.password)? {
            return Err(ApiError::new(
                ErrorCode::Unauthorized,
                "Invalid credentials",
                ErrorContext::default()
            ));
        }

        // Generate access token
        let access_token = Self::generate_token(&user)?;

        // Generate refresh token
        let refresh_token = refresh_repo.create_for_user(&mut conn, user.id).await?;

        Ok((access_token, refresh_token, user))
    }

    /// Register a new user
    pub async fn register(
        pool: &DbPool,
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_number: &str,
        password: &str,
        org_id: Uuid,
    ) -> Result<User> {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            ApiError::new(
                ErrorCode::DatabaseError,
                "Database connection error",
                ErrorContext::default()
            )
        })?;

        let user_repo = UserRepositoryImpl;

        // Check if user already exists
        if let Some(_) = user_repo.find_by_email(&mut conn, email).await? {
            return Err(ApiError::new(
                ErrorCode::ValidationError,
                "Email already registered",
                ErrorContext::default()
            ));
        }

        // Create user
        user_repo.create_with_password(
            &mut conn,
            first_name,
            last_name,
            email,
            phone_number,
            password,
            org_id,
        )
        .await
    }
} 