use actix_web::http::header;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Cors {
    allowed_origin: String,
}

impl Cors {
    pub fn new(allowed_origin: String) -> Self {
        Self { allowed_origin }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Cors
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CorsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CorsMiddleware {
            service,
            allowed_origin: self.allowed_origin.clone(),
        })
    }
}

#[derive(Default)]
pub struct CorsMiddleware<S> {
    service: S,
    allowed_origin: String,
}

impl<S, B> Service<ServiceRequest> for CorsMiddleware<S>
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
        let fut = self.service.call(req);
        let allowed_origin = self.allowed_origin.clone();

        Box::pin(async move {
            let mut res = fut.await?;
            
            // Add CORS headers
            let headers = res.headers_mut();
            
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::HeaderValue::from_str(&allowed_origin).unwrap(),
            );
            
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                header::HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
            );
            
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                header::HeaderValue::from_static(
                    "Content-Type, Authorization, Accept, X-Requested-With",
                ),
            );
            
            headers.insert(
                header::ACCESS_CONTROL_MAX_AGE,
                header::HeaderValue::from_static("3600"),
            );
            
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                header::HeaderValue::from_static("true"),
            );

            Ok(res)
        })
    }
} 