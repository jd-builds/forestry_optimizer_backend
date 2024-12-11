pub mod common;
pub mod integration;
pub mod unit;
pub mod performance;

use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is called exactly once before any tests
pub fn setup() {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok();
        
        // Verify required environment variables are set
        let required_vars = [
            "DATABASE_URL",
            "RUST_LOG",
            "JWT_SECRET",
            "ENVIRONMENT"
        ];

        for var in required_vars {
            if std::env::var(var).is_err() {
                panic!("{} should be set", var);
            }
        }
    });
}

/// Test module organization
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_setup() {
        setup();
        assert!(std::env::var("DATABASE_URL").is_ok());
        assert!(std::env::var("JWT_SECRET").is_ok());
        assert!(std::env::var("ENVIRONMENT").is_ok());
    }

    #[test]
    fn test_module_organization() {
        // Just verifying module structure exists
        assert!(true);
    }
}
