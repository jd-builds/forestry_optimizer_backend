use crate::{
    common::error::{ApiError, ErrorCode},
    domain::models::user::Role,
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    HttpMessage,
};
use futures::future::{ready, Ready};
use std::pin::Pin;
use std::future::Future;
use tracing::error;

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
    type Transform = RequireRoleMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequireRoleMiddleware {
            service,
            required_role: self.0.clone(),
        }))
    }
}

pub struct RequireRoleMiddleware<S> {
    service: S,
    required_role: Role,
}

impl<S, B> Service<ServiceRequest> for RequireRoleMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Get user role from request extensions
        let user_role = match req.extensions().get::<Role>() {
            Some(role) => *role,
            None => {
                error!("No role found in request extensions");
                return Box::pin(ready(Err(ApiError::new(
                    ErrorCode::Unauthorized,
                    "Missing authentication",
                    Default::default(),
                ).into())));
            }
        };

        // Check if user has required role
        if user_role >= self.required_role {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(ready(Err(ApiError::new(
                ErrorCode::Forbidden,
                "Insufficient permissions",
                Default::default(),
            ).into())))
        }
    }
} 