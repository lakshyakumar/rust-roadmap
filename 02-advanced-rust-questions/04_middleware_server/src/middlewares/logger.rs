use crate::middleware::Middleware;
use crate::types::Response;

pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn handle(&self, req: &str, client_ip: &str, next: &dyn Fn(&str) -> Response) -> Response {
        let mut parts = req.split_whitespace();
        let response = next(req);
        if let (Some(method), Some(path)) = (parts.next(), parts.next()) {
            println!(
                "[Logger] {} {}, status: {}, from {} ",
                method, path, response.status, client_ip
            )
        }
        response
    }
}
