//! Authentication middleware implementation
//! 
//! This module provides middleware for authenticating requests using JWT tokens
//! and enforcing role-based access control.

use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform}, Error, FromRequest, HttpMessage, HttpRequest
};
use futures_util::future::LocalBoxFuture;
use crate::{
    services::auth::{AuthService, Claims},
    errors::{ApiError, ErrorCode, ErrorContext},
    db::models::auth::Role,
};
use tracing::error;

/// Extractor for authenticated user claims
/// 
/// This extractor provides easy access to the authenticated user's claims
/// in route handlers. It automatically extracts claims from request extensions
/// that were previously stored by the AuthMiddleware.
/// 
/// # Example
/// ```rust
/// async fn handler(auth: AuthenticatedUser) -> HttpResponse {
///     let user_id = auth.sub;
///     let org_id = auth.org_id;
///     // ... use claims data ...
/// }
/// ```
#[allow(unused)]
pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get claims from request extensions
        let claims = req.extensions()
            .get::<Claims>()
            .cloned();

        match claims {
            Some(claims) => ready(Ok(AuthenticatedUser(claims))),
            None => ready(Err(ApiError::new(
                ErrorCode::Unauthorized,
                "Missing authentication",
                ErrorContext::default(),
            ).into()))
        }
    }
}

#[allow(unused)]
// Add convenience methods to access claims data
impl AuthenticatedUser {
    /// Get the authenticated user's ID
    pub fn user_id(&self) -> &str {
        &self.0.sub
    }

    /// Get the authenticated user's organization ID
    pub fn org_id(&self) -> &str {
        &self.0.org_id
    }

    /// Get the authenticated user's role
    pub fn role(&self) -> &str {
        &self.0.role
    }

    /// Get the underlying claims
    pub fn claims(&self) -> &Claims {
        &self.0
    }
}

/// Role-based authorization middleware
pub struct RequireRole(pub Role);

impl<S, B> Transform<S, ServiceRequest> for RequireRole
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleMiddleware {
            service,
            role: self.0,
        }))
    }
}

pub struct RoleMiddleware<S> {
    service: S,
    role: Role,
}

impl<S, B> Service<ServiceRequest> for RoleMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract claims from request extensions
        let claims = match req.extensions().get::<Claims>().cloned() {
            Some(claims) => claims,
            None => {
                return Box::pin(ready(Err(ApiError::new(
                    ErrorCode::Unauthorized,
                    "Missing authentication",
                    ErrorContext::default(),
                )
                .into())));
            }
        };

        // Parse role from claims - handle both "Admin" and "ADMIN" formats
        let user_role = match claims.role.to_uppercase().as_str() {
            "ADMIN" => Role::Admin,
            "MANAGER" => Role::Manager,
            "OPERATOR" => Role::Operator,
            _ => {
                error!("Invalid role in claims: {}", claims.role);
                return Box::pin(ready(Err(ApiError::new(
                    ErrorCode::Unauthorized,
                    "Invalid role",
                    ErrorContext::default(),
                )
                .into())));
            }
        };

        // Check if user has required role
        match (user_role, self.role) {
            (Role::Admin, _) => (),  // Admin can access everything
            (Role::Manager, Role::Manager | Role::Operator) => (),  // Manager can access Manager and Operator routes
            (Role::Operator, Role::Operator) => (),  // Operator can only access Operator routes
            _ => {
                return Box::pin(ready(Err(ApiError::new(
                    ErrorCode::Forbidden,
                    "Insufficient permissions",
                    ErrorContext::default(),
                )
                .into())));
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

/// JWT authentication middleware
pub struct AuthMiddleware<S> {
    service: S,
}

/// Authentication middleware factory
pub struct Auth;

impl Auth {
    pub fn new() -> Self {
        Auth
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract bearer token from Authorization header
        let auth_header = req.headers().get("Authorization");
        let token = match auth_header {
            Some(value) => {
                match value.to_str() {
                    Ok(header) => {
                        if !header.starts_with("Bearer ") {
                            return Box::pin(ready(Err(ApiError::new(
                                ErrorCode::Unauthorized,
                                "Invalid authorization header",
                                ErrorContext::default(),
                            )
                            .into())));
                        }
                        &header[7..]
                    }
                    Err(_) => {
                        return Box::pin(ready(Err(ApiError::new(
                            ErrorCode::Unauthorized,
                            "Invalid authorization header",
                            ErrorContext::default(),
                        )
                        .into())));
                    }
                }
            }
            None => {
                return Box::pin(ready(Err(ApiError::new(
                    ErrorCode::Unauthorized,
                    "Missing authorization header",
                    ErrorContext::default(),
                )
                .into())));
            }
        };

        // Validate token and extract claims
        match AuthService::validate_token(token) {
            Ok(claims) => {
                // Store claims in request extensions
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => Box::pin(ready(Err(e.into()))),
        }
    }
} 