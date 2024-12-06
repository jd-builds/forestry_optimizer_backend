use crate::{
    database::models::auth::User,
    error::{Result, ApiError, ErrorCode, ErrorContext},
    utils::Config,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
    errors::ErrorKind as JwtErrorKind,
};
use tracing::error;
use super::claims::Claims;

const JWT_EXPIRATION: i64 = 60 * 60; // 1 hour in seconds

/// Token management functionality
pub struct TokenManager;

impl TokenManager {
    /// Generate a new JWT token for a user
    pub fn generate_token(user: &User, config: &Config) -> Result<String> {
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
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
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
    pub fn validate_token(token: &str, config: &Config) -> Result<Claims> {
        let validation = Validation::default();

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
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
}