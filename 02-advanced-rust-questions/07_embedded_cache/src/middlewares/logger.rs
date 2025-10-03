use std::future::Future;
use std::pin::Pin;

use crate::middleware::Middleware;
use crate::types::Response;

pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn handle(
        &self,
        req: &str,
        client_ip: &str,
        next: &dyn for<'a> Fn(&'a str) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        let req = req.to_string();
        let client_ip = client_ip.to_string();
        let fut = next(&req);
        Box::pin(async move {
            let mut parts = req.split_whitespace();
            let response = fut.await;
            if let (Some(method), Some(path)) = (parts.next(), parts.next()) {
                println!(
                    "[Logger] {} {}, status: {}, from {} ",
                    method, path, response.status, client_ip
                )
            }
            response
        })
    }
}
