use crate::{middlewares, types::Response};

pub trait Middleware: Send + Sync {
    fn handle(&self, req: &str, client_ip: &str, next: &dyn Fn(&str) -> Response) -> Response;
}

pub fn run_chain<'a>(
    req: &str,
    client_ip: &str,
    middlewares: &'a [&dyn Middleware],
    handler: impl Fn(&str) -> Response,
) -> Response {
    fn call<'a>(
        req: &str,
        client_ip: &str,
        middlewares: &'a [&dyn Middleware],
        handler: &dyn Fn(&str) -> Response,
    ) -> Response {
        if let Some((first, rest)) = middlewares.split_first() {
            first.handle(req, client_ip, &|r| call(r, client_ip, rest, handler))
        } else {
            handler(req)
        }
    }

    call(req, client_ip, middlewares, &handler)
}
