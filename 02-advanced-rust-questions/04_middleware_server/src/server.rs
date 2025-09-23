use std::sync::Arc;

use crate::handlers;
use crate::middleware::{self, Middleware};
use crate::middlewares::logger::LoggerMiddleware;
use crate::middlewares::rate_limiting::TokenBucketMiddleware;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::types::Response;

pub async fn run() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    println!("Listening on port 7878");

    let middlewares: Vec<Arc<dyn Middleware>> = vec![
        Arc::new(LoggerMiddleware),
        Arc::new(TokenBucketMiddleware::new(5, 1)),
    ];

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let middlewares = middlewares.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            if let Ok(n) = socket.read(&mut buf).await {
                if n == 0 {
                    return;
                }

                let req = String::from_utf8_lossy(&buf[..n]);
                let middleware_ref: Vec<&dyn Middleware> =
                    middlewares.iter().map(|m| m.as_ref()).collect();
                let res =
                    middleware::run_chain(&req, &(addr.ip().to_string()), &middleware_ref, |req| {
                        route_request(req)
                    });
                let _ = socket.write_all(res.into_http().as_bytes()).await;
            }
        });
    }
}

pub fn route_request(req: &str) -> Response {
    if req.starts_with("GET /health") {
        handlers::health::handle()
    } else if req.starts_with("GET /hello/") {
        handlers::hello::handle(req)
    } else if req.starts_with("POST /user") {
        handlers::user::handle(req)
    } else {
        Response {
            status: 404,
            content_type: "text/plain".into(),
            body: "Not Found".into(),
        }
    }
}
