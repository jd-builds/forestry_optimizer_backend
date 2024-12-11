//! Request validation middleware
//! 
//! This middleware provides automatic validation of request payloads
//! against defined validation rules. It ensures that all requests contain
//! valid data before they reach the handlers.
//! 
//! # Features
//! 
//! - Automatic validation of request bodies
//! - Type-safe validation using generics
//! - Custom validation rules through traits
//! - Early rejection of invalid requests
//! - Detailed validation error messages
//! 
//! # Example
//! 
//! ```rust
//! use actix_web::{web, App, HttpResponse, test};
//! use optimizer::api::middleware::validation::{ValidateRequest, RequestValidate};
//! use optimizer::error::ApiError;
//! use serde::{Deserialize, Serialize};
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! struct CreateUser {
//!     name: String,
//!     email: String,
//! }
//! 
//! impl RequestValidate for CreateUser {
//!     fn validate(&self) -> Result<(), ApiError> {
//!         if self.name.is_empty() {
//!             return Err(ApiError::validation("Name cannot be empty", None));
//!         }
//!         Ok(())
//!     }
//! }
//! 
//! async fn create_user(user: web::Json<CreateUser>) -> HttpResponse {
//!     HttpResponse::Ok().json(user.0)
//! }
//! 
//! #[actix_web::test]
//! async fn test_validation() {
//!     let app = test::init_service(
//!         App::new()
//!             .wrap(ValidateRequest::<CreateUser>::new())
//!             .service(
//!                 web::resource("/users")
//!                     .route(web::post().to(create_user))
//!             )
//!     ).await;
//! }
//! ```

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Json;
use actix_web::{Error, FromRequest};
use futures::future::{ready, Ready};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use crate::error::ApiError;

/// Request validation middleware configuration
#[derive(Clone)]
pub struct ValidateRequest<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for ValidateRequest<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ValidateRequest<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<S, B, T> Transform<S, ServiceRequest> for ValidateRequest<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
    T: DeserializeOwned + RequestValidate + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidateRequestMiddleware<S, T>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidateRequestMiddleware {
            service,
            _phantom: PhantomData,
        }))
    }
}

#[derive(Clone)]
pub struct ValidateRequestMiddleware<S, T> {
    service: S,
    _phantom: PhantomData<T>,
}

impl<S, B, T> Service<ServiceRequest> for ValidateRequestMiddleware<S, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
    T: DeserializeOwned + RequestValidate + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let fut = async move {
            let body = Json::<T>::extract(req.request()).await?;
            body.validate().map_err(Error::from)?;
            svc.call(req).await
        };
        Box::pin(fut)
    }
}

/// Trait for implementing custom validation rules
pub trait RequestValidate {
    /// Validates the request payload
    fn validate(&self) -> Result<(), ApiError>;
} 