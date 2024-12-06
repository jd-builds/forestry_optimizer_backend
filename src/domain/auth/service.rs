use crate::{
    db::{
        models::auth::{User, RefreshToken},
        repositories::auth::{UserRepositoryImpl, RefreshTokenRepositoryImpl, CreateUserParams, UserRepository, RefreshTokenRepository},
        repositories::Repository,
        DbPool, connection,
    },
    error::{Result, ApiError},
    utils::Config,
    api::utils::{ApiResponse, ApiResponseBuilder},
};
use super::{
    tokens::TokenManager,
    validation::AuthValidator,
};
use uuid::Uuid;
use chrono::Utc;

/// Authentication service for user management and authentication
pub struct AuthService;

impl AuthService {
    /// Refresh an access token using a refresh token
    pub async fn refresh_token(pool: &DbPool, refresh_token: &str, config: &Config) -> Result<ApiResponse<(String, RefreshToken)>> {
        let mut conn = connection::get_connection(pool)?;

        let repo = RefreshTokenRepositoryImpl;
        // Find the refresh token
        let token = repo.find_by_token(&mut conn, refresh_token)
            .await?
            .ok_or_else(|| ApiError::unauthorized("Invalid refresh token"))?;

        // Check if token is expired
        if token.expires_at < Utc::now() {
            return Err(ApiError::unauthorized("Refresh token expired"));
        }

        let user_repo = UserRepositoryImpl;
        // Find the user
        let user = user_repo.find_by_id(&mut conn, token.user_id).await?;

        // Generate new access token
        let access_token = TokenManager::generate_token(&user, config)?;

        // Create new refresh token
        let new_refresh_token = repo.create_for_user(&mut conn, user.id).await?;

        // Revoke old refresh token
        repo.revoke_all_for_user(&mut conn, user.id).await?;

        Ok(ApiResponseBuilder::success()
            .with_message("Token refreshed successfully")
            .with_data((access_token, new_refresh_token))
            .build())
    }

    /// Login a user and generate tokens
    pub async fn login(
        pool: &DbPool,
        email: &str,
        password: &str,
        config: &Config,
    ) -> Result<ApiResponse<(String, RefreshToken, User)>> {
        let mut conn = connection::get_connection(pool)?;

        let user_repo = UserRepositoryImpl;
        let refresh_repo = RefreshTokenRepositoryImpl;

        // Validate credentials and get user
        let user = AuthValidator::validate_login(&mut conn, &user_repo, email, password).await?;

        // Generate access token
        let access_token = TokenManager::generate_token(&user, config)?;

        // Generate refresh token
        let refresh_token = refresh_repo.create_for_user(&mut conn, user.id).await?;

        Ok(ApiResponseBuilder::success()
            .with_message("Login successful")
            .with_data((access_token, refresh_token, user))
            .build())
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
    ) -> Result<ApiResponse<User>> {
        let mut conn = connection::get_connection(pool)?;

        let user_repo = UserRepositoryImpl;

        let params = CreateUserParams {
            first_name,
            last_name,
            email,
            phone_number,
            password,
            org_id,
        };

        // Validate registration input
        AuthValidator::validate_registration(&mut conn, &user_repo, &params).await?;
        
        // Create user
        let user = user_repo.create_with_password(&mut conn, params).await?;

        Ok(ApiResponseBuilder::success()
            .with_message("User registered successfully")
            .with_data(user)
            .build())
    }
}