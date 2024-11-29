use crate::common::error::{ApiError, ErrorCode, Result};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher as _, PasswordVerifier as _,
        SaltString,
    },
    Argon2,
};

pub struct PasswordHasher {
    argon2: Argon2<'static>,
}

impl PasswordHasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        
        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| ApiError::new(
                ErrorCode::InternalError,
                "Failed to hash password",
                Default::default(),
            ))
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| ApiError::new(
                ErrorCode::InternalError,
                "Invalid password hash",
                Default::default(),
            ))?;

        Ok(self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}