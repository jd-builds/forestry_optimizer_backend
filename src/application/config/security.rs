pub struct SecurityConfig {
    pub jwt_secret: String,
    pub token_expiration: i64,
    pub cors_allowed_origin: String,
} 