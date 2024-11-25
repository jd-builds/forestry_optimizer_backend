use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Json;
use actix_web::{Error, FromRequest};
use futures::future::{ready, Ready};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

pub struct ValidateRequest<T> {
    _phantom: PhantomData<T>,
}

impl<T> ValidateRequest<T> {
    #[allow(dead_code)]
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

pub trait RequestValidate {
    fn validate(&self) -> Result<(), Error>;
} 