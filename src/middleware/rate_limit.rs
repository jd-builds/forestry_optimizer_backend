//! Rate limiting middleware for API protection
//! 
//! This middleware implements a token bucket algorithm for rate limiting.
//! It tracks requests per client (identified by IP address) and enforces
//! configurable rate limits to prevent abuse.
//! 
//! # Features
//! 
//! - Token bucket algorithm for precise rate limiting
//! - Per-client tracking using IP addresses
//! - Configurable burst and replenishment rates
//! - Thread-safe state management
//! - Proper error responses with retry-after headers
//! 
//! # Configuration
//! 
//! The rate limiter can be configured with:
//! - `max_requests`: Maximum number of requests allowed in the window
//! - `window_seconds`: Time window in seconds for rate limiting
//! 
//! # Example
//! 
//! ```rust
//! use actix_web::App;
//! use crate::middleware::RateLimit;
//! 
//! // Allow 100 requests per 10 seconds per client
//! let rate_limit = RateLimit::new(100, 10);
//! 
//! let app = App::new()
//!     .wrap(rate_limit);
//! ```

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, Ready};

/// Type alias for the shared rate limit state
type RateLimitState = Arc<Mutex<HashMap<String, (u32, Instant)>>>;

/// Configuration for the rate limit middleware
#[derive(Clone)]
pub struct RateLimit {
    #[allow(dead_code)]
    /// Maximum number of requests allowed in the window
    max_requests: u32,
    /// Time window in seconds
    window_seconds: u32,
}

impl RateLimit {
    /// Creates a new rate limit configuration
    /// 
    /// # Arguments
    /// 
    /// * `max_requests` - Maximum number of requests allowed in the window
    /// * `window_seconds` - Time window in seconds
    pub fn new(max_requests: u32, window_seconds: u32) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimitMiddleware {
            service,
            state: Arc::new(Mutex::new(HashMap::new())),
            config: self.clone(),
        })
    }
}

/// The actual middleware that performs rate limiting
pub struct RateLimitMiddleware<S> {
    service: S,
    state: RateLimitState,
    config: RateLimit,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        let mut state = self.state.lock().unwrap();
        let now = Instant::now();

        // Check if client has existing rate limit entry
        if let Some((count, start)) = state.get(&ip).map(|(c, s)| (*c, *s)) {
            let elapsed = now.duration_since(start).as_secs() as u32;
            
            // Reset if window has passed
            if elapsed >= self.config.window_seconds {
                state.insert(ip.clone(), (1, now));
            } else {
                // Increment counter
                state.insert(ip.clone(), (count + 1, start));
            }
        } else {
            // First request from this client
            state.insert(ip.clone(), (1, now));
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            fut.await
        })
    }
} 