use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use crate::{
    db::models::auth::Role,
    error::{ApiError, ErrorCode, ErrorContext},
    domain::auth::Claims,
};
use tracing::error;

/// Middleware for requiring a specific role
#[derive(Clone)]
pub struct RequireRole(pub Role);

/// Middleware for requiring authentication
#[derive(Clone)]
pub struct RequireAuth;

pub struct RoleMiddleware<S> {
    service: S,
    role: Role,
}

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

impl<S, B> Transform<S, ServiceRequest> for RequireAuth
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
            role: Role::Operator,
        }))
    }
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

        // Parse role from claims
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