use std::future::{ready, Ready, Future};
use std::pin::Pin;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error,
};
use crate::common::error::{ApiError, ErrorCode};
use tracing::error;

pub struct ErrorHandlerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ErrorHandlerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorHandlerMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorHandlerMiddlewareService { service }))
    }
}

pub struct ErrorHandlerMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorHandlerMiddlewareService<S>
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
            match fut.await {
                Ok(res) => Ok(res),
                Err(e) => {
                    error!(error = %e, "Request processing error");
                    
                    let api_error = match e.as_response_error() {
                        err if err.status_code() == StatusCode::NOT_FOUND => {
                            ApiError::new(
                                ErrorCode::NotFound,
                                "Resource not found",
                                Default::default(),
                            )
                        }
                        err if err.status_code() == StatusCode::INTERNAL_SERVER_ERROR => {
                            ApiError::new(
                                ErrorCode::InternalError,
                                "Internal server error",
                                Default::default(),
                            )
                        }
                        _ => ApiError::new(
                            ErrorCode::InternalError,
                            "Unexpected error",
                            Default::default(),
                        ),
                    };
                    
                    Err(api_error.into())
                }
            }
        })
    }
}

impl Default for ErrorHandlerMiddleware {
    fn default() -> Self {
        Self
    }
} 