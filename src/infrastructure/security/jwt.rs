use crate::common::error::{ApiError, ErrorCode, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,      // Subject (user ID)
    pub org_id: Uuid,   // Organization ID
    pub role: String,   // User role
    pub exp: i64,       // Expiration time
    pub iat: i64,       // Issued at
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub fn create_token(&self, user_id: Uuid, org_id: Uuid, role: String, expires_in: i64) -> Result<String> {
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            org_id,
            role,
            iat: now,
            exp: now + expires_in,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| ApiError::new(
                ErrorCode::InternalError,
                "Failed to create token",
                Default::default(),
            ))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| ApiError::new(
            ErrorCode::Unauthorized,
            "Invalid token",
            Default::default(),
        ))
    }
}