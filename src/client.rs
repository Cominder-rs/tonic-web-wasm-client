use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Future;
use http::{Request, Response};
use tonic::body::BoxBody;
use tower_service::Service;

use crate::{call::call, Error, ResponseBody};

/// `grpc-web` based transport layer for `tonic` clients
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    headers: Vec<(String, String)>
}

impl Client {
    /// Creates a new client
    pub fn new(base_url: String) -> Self {
        Self { base_url, headers: vec![] }
    }

    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = headers;
        self
    }

}

impl Service<Request<BoxBody>> for Client {
    type Response = Response<ResponseBody>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<BoxBody>) -> Self::Future {
        let headers = self.headers.clone();
        let base_url = self.base_url.clone();
        
        Box::pin(call(base_url, request, headers))
    }
}
