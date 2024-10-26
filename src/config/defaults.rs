use super::environment::Environment;

pub fn default_environment() -> Environment {
    Environment::Development
}

pub fn default_log_level() -> String {
    "debug".to_string()
}

pub fn default_host() -> String {
    "0.0.0.0".to_string()
}

pub fn default_port() -> u16 {
    8080
}
