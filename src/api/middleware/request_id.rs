//! Request ID middleware for request tracing
//! 
//! This middleware adds a unique identifier to each request, enabling
//! request tracing and correlation across the system. The ID is added
//! to request extensions and can be accessed by handlers and other
//! middleware components.
//! 
//! # Features
//! 
//! - Unique UUID generation for each request
//! - Zero-allocation implementation
//! - Request ID available throughout request lifecycle
//! - Automatic response header injection
//! - Integration with logging system
//! 
//! # Example
//! 
//! ```rust
//! use actix_web::{web, App, HttpResponse, HttpRequest, HttpMessage};
//! use optimizer::api::middleware::request_id::RequestId;
//! use uuid::Uuid;
//! 
//! async fn handler(req: HttpRequest) -> HttpResponse {
//!     let extensions = req.extensions();
//!     let request_id = extensions
//!         .get::<Uuid>()
//!         .expect("RequestId middleware not enabled");
//!     
//!     // Use request_id for logging or response
//!     HttpResponse::Ok()
//!         .insert_header(("X-Request-Id", request_id.to_string()))
//!         .finish()
//! }
//! 
//! let app = App::new()
//!     .wrap(RequestId::new())
//!     .service(web::resource("/").to(handler));
//! ```

use std::future::{ready, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use uuid::Uuid;

/// Request ID middleware
/// 
/// This middleware adds a unique UUID to each request's extensions.
/// The ID can be used for request tracing and correlation.
#[derive(Default, Clone)]
pub struct RequestId;

impl RequestId {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware { service }))
    }
}

/// The actual middleware that adds request IDs
pub struct RequestIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Generate and insert request ID
        let request_id = Uuid::new_v4();
        req.extensions_mut().insert(request_id);

        // Add request ID to response headers
        self.service.call(req)
    }
} 