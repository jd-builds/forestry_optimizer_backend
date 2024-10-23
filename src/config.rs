use crate::error::{AppError, AppResult};
use std::env;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> AppResult<Config> {
    Ok(Config {
        database_url: env::var("DATABASE_URL")?,
        host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| AppError::Validation("PORT must be a number".to_string()))?,
    })
}
