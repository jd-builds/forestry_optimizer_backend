use std::env;

pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Config {
    Config {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().expect("PORT must be a number"),
    }
}
