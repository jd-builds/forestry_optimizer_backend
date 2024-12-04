use super::environment::Environment;

pub fn default_environment() -> Environment {
    Environment::Development
}

pub fn default_host() -> String {
    "0.0.0.0".to_string()
}

pub fn default_port() -> u16 {
    8080
}

pub fn default_jwt_secret() -> String {
    "your-super-secret-key-for-development".to_string()
}
