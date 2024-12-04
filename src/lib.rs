pub mod api;
pub mod config;
pub mod db;
pub mod docs;
pub mod errors;
pub mod middleware;
pub mod services;

// Re-export commonly used types
pub use config::Config;
pub use errors::{ApiError, ErrorCode, Result};
pub use server::run;

// Constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

// Private modules
mod server;
