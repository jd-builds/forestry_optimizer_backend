use crate::{
    common::error::Result,
    domain::models::user::{Role, User},
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AuthService: Send + Sync {
    /// Register a new user
    async fn register_user(
        &self,
        first_name: String,
        last_name: String,
        email: String,
        phone_number: String,
        password: String,
        org_id: Uuid,
        role: Role,
    ) -> Result<User>;
    
    /// Authenticate user and return JWT token
    async fn login(&self, email: String, password: String) -> Result<String>;
    
    /// Verify JWT token and return user
    async fn verify_token(&self, token: &str) -> Result<User>;
    
    /// Get user by ID
    async fn get_user(&self, id: Uuid) -> Result<User>;
    
    /// Update user's password
    async fn update_password(&self, id: Uuid, old_password: String, new_password: String) -> Result<()>;
    
    /// Update user's role (admin only)
    async fn update_role(&self, id: Uuid, new_role: Role) -> Result<User>;
} 