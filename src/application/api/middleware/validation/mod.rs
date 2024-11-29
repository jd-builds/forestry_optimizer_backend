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
//! use serde::Deserialize;
//! use crate::middleware::ValidateRequest;
//! 
//! #[derive(Deserialize)]
//! struct CreateUser {
//!     name: String,
//!     email: String,
//! }
//! 
//! impl RequestValidate for CreateUser {
//!     fn validate(&self) -> Result<(), Error> {
//!         if self.name.is_empty() {
//!             return Err(Error::from("Name cannot be empty"));
//!         }
//!         Ok(())
//!     }
//! }
//! 
//! // In your route configuration:
//! App::new()
//!     .service(
//!         web::resource("/users")
//!             .wrap(ValidateRequest::<CreateUser>::new())
//!             .route(web::post().to(create_user))
//!     )
//! ```

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Json;
use actix_web::{Error, FromRequest};
use futures::future::{ready, Ready};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

/// Request validation middleware configuration
/// 
/// Generic type parameter `T` represents the type of payload to validate.
/// The type must implement `DeserializeOwned` and `RequestValidate`.
pub struct ValidateRequest<T> {
    _phantom: PhantomData<T>,
}

impl<T> ValidateRequest<T> {
    #[allow(dead_code)]
    /// Creates a new validation middleware for type T
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

/// The actual middleware that performs validation
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
            // Extract and validate the JSON body
            let body = Json::<T>::extract(req.request()).await?;
            body.validate()?;
            
            // Call the next service
            svc.call(req).await
        };

        Box::pin(fut)
    }
}

/// Trait for implementing custom validation rules
/// 
/// Types that can be validated must implement this trait.
/// The `validate` method should return `Ok(())` if the data is valid,
/// or an appropriate error if validation fails.
pub trait RequestValidate {
    /// Validates the request payload
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` if validation passes
    /// * `Err(Error)` with appropriate error message if validation fails
    fn validate(&self) -> Result<(), Error>;
} 