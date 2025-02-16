use std::future::{ready, Ready};
use std::time::Instant;

use actix_http::header::{HeaderName, HeaderValue};
use actix_http::HttpMessage;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, web};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header;
use futures::future::LocalBoxFuture;
use log::info;

use cerium::error::web::OrcaError;
use cerium::utils::uuid::request_uuid;

use crate::server::context::request::RequestContext;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middlewares call method gets called with normal request.
#[derive(Debug, Clone, Default)]
pub struct RequestHandler;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RequestHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestHandlerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestHandlerMiddleware { service }))
    }
}

pub struct RequestHandlerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let request_id = request_uuid();
        let start_time = Instant::now();
        info!("Starting the Request {}", &request_id.clone());
        // let authorization = req.headers().get(header::AUTHORIZATION);
        // if authorization.is_none() {
        //     return Box::pin(async { Err(ErrorUnauthorized("err")) });
        // }
        let rc = RequestContext::new(&req);
        req.extensions_mut().insert(rc);
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut _response = fut.await;
            // res.headers_mut().insert(HeaderName::from_static(REQUEST_ID_HEADER),
            //                          HeaderValue::from_str(&request_id).unwrap());
            info!("Completed Request after - {:?}", start_time.elapsed());
            // rc.end_request();
            _response
        })
    }
}

pub fn map_io_error(e: std::io::Error) -> OrcaError {
    match e.kind() {
        _ => OrcaError::IoError(e),
    }
}