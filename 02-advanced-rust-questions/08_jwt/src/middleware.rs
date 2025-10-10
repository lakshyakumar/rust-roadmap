use crate::types::Response;
use std::future::Future;
use std::pin::Pin;

pub trait Middleware: Send + Sync {
    fn handle(
        &self,
        req: &str,
        client_ip: &str,
        next: &dyn Fn(&str) -> Pin<Box<dyn Future<Output = Response> + Send>>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
}

pub async fn run_chain<'a, F, Fut>(
    req: &str,
    client_ip: &str,
    middlewares: &'a [&dyn Middleware],
    handler: F,
) -> Response
where
    F: Fn(&str) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    fn call<'a>(
        req: &str,
        client_ip: &str,
        middlewares: &'a [&dyn Middleware],
        handler: &dyn Fn(&str) -> Pin<Box<dyn Future<Output = Response> + Send>>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send>> {
        if let Some((first, rest)) = middlewares.split_first() {
            first.handle(req, client_ip, &move |r| call(r, client_ip, rest, handler))
        } else {
            handler(req)
        }
    }

    call(req, client_ip, middlewares, &|r| Box::pin(handler(r))).await
}
