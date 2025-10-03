use sled;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::cache::LRUCache;
use crate::handlers;
use crate::handlers::user::UserResponse;
use crate::middleware::{self, Middleware};
use crate::middlewares::logger::LoggerMiddleware;
use crate::middlewares::rate_limiting::TokenBucketMiddleware;
use mongodb::{Client, Database};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::types::Response;

pub type UserCache = sled::Db;

pub async fn run() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongodb_db = std::env::var("MONGODB_DB").unwrap_or_else(|_| "my_app".to_string());
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let db = Arc::new(client.database(&mongodb_db));
    let cache = sled::open("user_cache").expect("open sled db");

    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    println!("Listening on port 7878");

    let middlewares: Vec<Arc<dyn Middleware>> = vec![
        Arc::new(LoggerMiddleware),
        Arc::new(TokenBucketMiddleware::new(5, 1)),
    ];

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let middlewares = middlewares.clone();
        let db = db.clone();
        let cache_clone = cache.clone();
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            if let Ok(n) = socket.read(&mut buf).await {
                if n == 0 {
                    return;
                }

                let req = String::from_utf8_lossy(&buf[..n]);
                let middleware_ref: Vec<&dyn Middleware> =
                    middlewares.iter().map(|m| m.as_ref()).collect();
                let cache_for_handler = cache_clone.clone();
                let handler: Box<
                    dyn Fn(&str) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync,
                > = Box::new(move |req: &str| {
                    let req_owned = req.to_string();
                    let db = db.clone();
                    let cache = cache_for_handler.clone();
                    Box::pin(async move { route_request(&req_owned, &db, &cache).await })
                });
                let res =
                    middleware::run_chain(&req, &(addr.ip().to_string()), &middleware_ref, handler)
                        .await;
                let _ = socket.write_all(res.into_http().as_bytes()).await;
            }
        });
    }
}

pub async fn route_request(req: &str, db: &Database, cache: &sled::Db) -> Response {
    if req.starts_with("GET /health") {
        handlers::health::handle().await
    } else if req.starts_with("GET /hello/") {
        handlers::hello::handle(req).await
    } else if req.starts_with("POST /user") {
        handlers::user::handle(req, db, cache).await
    } else if req.starts_with("GET /user") {
        handlers::user::get(req, db, cache).await
    } else {
        Response {
            status: 404,
            content_type: "text/plain".into(),
            body: "Not Found".into(),
        }
    }
}
