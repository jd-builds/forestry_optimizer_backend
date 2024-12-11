//! Performance and benchmark tests
//! Tests for measuring performance characteristics

pub mod benchmarks;
pub mod load;

pub use benchmarks::*;
pub use load::*;

// Common performance test constants
pub const PERFORMANCE_THRESHOLD_MS: u128 = 1000; // 1 second
pub const BATCH_SIZE: usize = 100;
pub const CONCURRENT_USERS: usize = 10; 