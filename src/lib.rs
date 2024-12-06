pub mod api;
pub mod utils;
pub mod database;
pub mod domain;
pub mod error;
pub mod server;

// Re-export commonly used types
pub use utils::Config;
pub use error::{ApiError, ErrorCode, Result};

// Constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
