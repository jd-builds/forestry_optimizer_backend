pub mod common;
pub mod api;
pub mod domain;
pub mod db;
pub mod integration;
pub mod performance;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::TestConfig;

    #[test]
    fn test_module_organization() {
        // Verify test configuration is loaded
        let config = TestConfig::new();
        assert!(!config.database_url.is_empty(), "Database URL should be configured");

        // Verify database connection works
        let _conn = common::TestDb::conn();
        // Connection successful if we get here (would panic on error)
    }

    #[test]
    fn test_environment_setup() {
        // Verify test environment variables
        assert!(std::env::var("DATABASE_URL").is_ok(), "DATABASE_URL should be set");
        assert!(std::env::var("RUST_LOG").is_ok(), "RUST_LOG should be set");
        assert!(std::env::var("JWT_SECRET").is_ok(), "JWT_SECRET should be set");
    }
}
