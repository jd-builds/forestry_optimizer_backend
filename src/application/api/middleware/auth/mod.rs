pub mod role;

use crate::{
    common::error::{ApiError, ErrorCode},
    domain::services::AuthService,
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error,
    HttpMessage,
};
use futures::future::{ready, Ready};
use std::{future::Future, pin::Pin, sync::Arc};
use tracing::error;

pub struct AuthMiddleware {
    auth_service: Arc<dyn AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            auth_service: self.auth_service.clone(),
        }))
    }
}

#[derive(Clone)]
pub struct AuthMiddlewareService<S> {
    service: S,
    auth_service: Arc<dyn AuthService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_service = self.auth_service.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // Skip auth for login and register endpoints
            if req.path().ends_with("/login") || req.path().ends_with("/register") {
                return service.call(req).await;
            }

            // Get token from Authorization header
            let token = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .ok_or_else(|| {
                    ApiError::new(
                        ErrorCode::Unauthorized,
                        "Missing or invalid authorization header",
                        Default::default(),
                    )
                })?;

            // Verify token and get user
            match auth_service.verify_token(token).await {
                Ok(user) => {
                    // Add user and role to request extensions
                    req.extensions_mut().insert(user.clone());
                    req.extensions_mut().insert(user.role);
                    service.call(req).await
                }
                Err(e) => {
                    error!(error = %e, "Failed to verify auth token");
                    Err(e.into())
                }
            }
        })
    }
} 