use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    Error, FromRequest, HttpMessage, HttpRequest, web,
};
use futures_util::future::LocalBoxFuture;
use crate::{
    services::auth::{AuthService, Claims},
    errors::{ApiError, ErrorCode, ErrorContext},
    config::Config,
};
use tracing::error;

/// Extractor for authenticated user claims
pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let claims = req.extensions().get::<Claims>().cloned();
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

impl AuthenticatedUser {
    pub fn user_id(&self) -> &str {
        &self.0.sub
    }

    pub fn org_id(&self) -> &str {
        &self.0.org_id
    }

    pub fn role(&self) -> &str {
        &self.0.role
    }

    pub fn claims(&self) -> &Claims {
        &self.0
    }
}

/// JWT authentication middleware
pub struct AuthMiddleware<S> {
    service: S,
}

/// Authentication middleware factory
#[derive(Clone)]
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
    type InitError = ();
    type Transform = AuthMiddleware<S>;
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
        let config = req.app_data::<web::Data<Config>>()
            .expect("Config not found in app data");
            
        let auth_header = req.headers().get("Authorization");
        
        let token = match auth_header {
            Some(value) => match value.to_str() {
                Ok(header) => {
                    if !header.starts_with("Bearer ") {
                        error!("Invalid auth header format: {}", header);
                        return Box::pin(ready(Err(ApiError::new(
                            ErrorCode::Unauthorized,
                            "Invalid authorization header",
                            ErrorContext::default(),
                        ).into())));
                    }
                    let token = header[7..].to_string();
                    token
                }
                Err(e) => {
                    error!("Failed to convert auth header to string: {}", e);
                    return Box::pin(ready(Err(ApiError::new(
                        ErrorCode::Unauthorized,
                        "Invalid authorization header",
                        ErrorContext::default(),
                    ).into())));
                }
            },
            None => {
                error!("Missing Authorization header");
                return Box::pin(ready(Err(ApiError::new(
                    ErrorCode::Unauthorized,
                    "Missing authorization header",
                    ErrorContext::default(),
                ).into())));
            }
        };

        match AuthService::validate_token(&token, &config) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => {
                error!("Token validation failed: {:?}", e);
                Box::pin(ready(Err(e.into())))
            }
        }
    }
} 