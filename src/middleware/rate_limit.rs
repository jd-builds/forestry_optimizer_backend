use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{ready, Ready};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimit {
    #[allow(dead_code)]
    requests_per_second: u32,
    burst_size: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            burst_size: 5,
        }
    }
}

impl RateLimit {
    pub fn new(requests_per_second: u32, burst_size: u32) -> Self {
        Self {
            requests_per_second,
            burst_size,
        }
    }
}

type RateLimitState = Arc<Mutex<HashMap<String, (Instant, u32)>>>;

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = RateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddleware {
            service,
            state: Arc::new(Mutex::new(HashMap::new())),
            config: self.clone(),
        }))
    }
}

pub struct RateLimitMiddleware<S> {
    service: S,
    state: RateLimitState,
    config: RateLimit,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();
        
        let state = self.state.clone();
        let config = self.config.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut state_lock = state.lock().await;
            let now = Instant::now();
            
            // Get current state for IP
            let current_state = state_lock.get(&ip).cloned();
            
            // Check rate limit
            if let Some((last_request_time, count)) = current_state {
                let time_passed = now.duration_since(last_request_time);
                
                if time_passed < Duration::from_secs(1) && count >= config.burst_size {
                    return Err(actix_web::error::ErrorTooManyRequests("Rate limit exceeded"));
                }
                
                state_lock.insert(ip, (now, count + 1));
            } else {
                state_lock.insert(ip, (now, 1));
            }
            
            // Drop the lock before awaiting the future
            drop(state_lock);
            
            fut.await
        })
    }
} 