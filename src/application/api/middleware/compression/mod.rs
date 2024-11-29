use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderValue, VARY},
    Error,
};
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Compression;

impl<S, B> Transform<S, ServiceRequest> for Compression
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CompressionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CompressionMiddleware { service })
    }
}

pub struct CompressionMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CompressionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Check if client accepts compression
        let accepts_compression = req
            .headers()
            .get("Accept-Encoding")
            .and_then(|h| h.to_str().ok())
            .map(|h| h.contains("gzip") || h.contains("br"))
            .unwrap_or(false);

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if accepts_compression {
                // Add Vary header
                res.headers_mut().insert(
                    VARY,
                    HeaderValue::from_static("Accept-Encoding"),
                );
            }

            Ok(res)
        })
    }
} 