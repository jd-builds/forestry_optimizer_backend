use uuid::Uuid;

pub struct TestAuth;

impl TestAuth {
    /// Creates a test JWT token
    pub fn create_test_token(_user_id: Uuid, _role: &str) -> String {
        // TODO: Implement JWT token creation for tests
        "test_token".to_string()
    }
} 