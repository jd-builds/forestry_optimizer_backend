use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}