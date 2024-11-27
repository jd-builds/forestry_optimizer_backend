//! Security headers middleware
//! 
//! This middleware adds various security headers to responses to protect against
//! common web vulnerabilities like XSS, clickjacking, and other attacks.

use std::future::{ready, Ready, Future};
use std::pin::Pin;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error,
};

/// Security headers middleware that adds various security-related headers to responses
#[derive(Default, Clone)]
pub struct SecurityHeaders;

impl SecurityHeaders {
    pub fn new() -> Self {
        SecurityHeaders
    }
}

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHeadersMiddleware { service }))
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
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
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            
            // Add security headers
            let headers = res.headers_mut();
            
            // Prevent XSS attacks
            headers.insert(
                header::X_XSS_PROTECTION,
                header::HeaderValue::from_static("1; mode=block"),
            );
            
            // Prevent clickjacking
            headers.insert(
                header::X_FRAME_OPTIONS,
                header::HeaderValue::from_static("DENY"),
            );
            
            // Prevent MIME type sniffing
            headers.insert(
                header::X_CONTENT_TYPE_OPTIONS,
                header::HeaderValue::from_static("nosniff"),
            );
            
            // Enable strict transport security (HSTS)
            headers.insert(
                header::STRICT_TRANSPORT_SECURITY,
                header::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
            );
            
            // Content Security Policy
            headers.insert(
                header::CONTENT_SECURITY_POLICY,
                header::HeaderValue::from_static(
                    "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline';",
                ),
            );
            
            // Referrer Policy
            headers.insert(
                header::REFERRER_POLICY,
                header::HeaderValue::from_static("strict-origin-when-cross-origin"),
            );
            
            // Permissions Policy
            headers.insert(
                header::HeaderName::from_static("permissions-policy"),
                header::HeaderValue::from_static(
                    "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()",
                ),
            );

            Ok(res)
        })
    }
} 