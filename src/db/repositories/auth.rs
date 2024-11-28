// This file is deprecated. All implementations have been moved to:
// - src/db/repositories/traits/auth.rs for traits
// - src/db/repositories/implementations/auth.rs for implementations

// Re-export the traits and implementations for backward compatibility
pub use crate::db::repositories::{
    traits::auth::{
        UserRepository,
        RefreshTokenRepository,
        PasswordResetTokenRepository,
        EmailVerificationTokenRepository,
    },
    implementations::auth::{
        UserRepositoryImpl,
        RefreshTokenRepositoryImpl,
        PasswordResetTokenRepositoryImpl,
        EmailVerificationTokenRepositoryImpl,
    },
}; 