use sled;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::Notify;
use tokio::time::{self, Duration};

use mongodb::{Client, Database};

use crate::cache::LRUCache;
use crate::handlers;
use crate::handlers::pubsub::PubSubHandler;
use crate::handlers::user::UserResponse;
use crate::middleware::{self, Middleware};
use crate::middlewares::logger::LoggerMiddleware;
use crate::middlewares::metrics::{self, MetricsMiddleware};
use crate::middlewares::rate_limiting::TokenBucketMiddleware;
use crate::pubsub::PubSubManager;
use crate::types::Response;
use crate::workers::pool::WorkerPool; // 👈 NEW: import WorkerPool
pub type UserCache = sled::Db;

pub async fn run() -> anyhow::Result<()> {
    // --- Setup phase ---
    let metrics = Arc::new(MetricsMiddleware::new());
    dotenvy::dotenv().ok();
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mongodb_db = std::env::var("MONGODB_DB").unwrap_or_else(|_| "my_app".to_string());
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let db = Arc::new(client.database(&mongodb_db));
    let cache = sled::open("user_cache").expect("open sled db");
    let pubsub_manager = Arc::new(tokio::sync::Mutex::new(PubSubManager::new()));
    let pubsub_handler = PubSubHandler::new(pubsub_manager.clone());

    // 👇 Initialize Worker Pool
    let worker_pool = Arc::new(WorkerPool::new(4)); // 4 async workers

    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    println!("🚀 Listening on port 7878");

    let middlewares: Vec<Arc<dyn Middleware>> = vec![
        Arc::new(LoggerMiddleware),
        Arc::new(TokenBucketMiddleware::new(5, 1)),
        metrics.clone(),
    ];

    // --- Shared shutdown signal ---
    let is_shutting_down = Arc::new(AtomicBool::new(false));
    let notify = Arc::new(Notify::new());

    // --- Setup signal listener ---
    {
        let shutdown_notify = notify.clone();
        let shutdown_flag = is_shutting_down.clone();

        tokio::spawn(async move {
            signal::ctrl_c().await.expect("failed to listen for event");
            println!("\n🧘 Graceful shutdown signal received...");
            shutdown_flag.store(true, Ordering::SeqCst);
            shutdown_notify.notify_waiters();
        });
    }

    // --- Start Cron Scheduler ---
    {
        let cron_flag = is_shutting_down.clone();
        let pool_clone = worker_pool.clone();

        tokio::spawn(async move {
            let interval_secs: u64 = std::env::var("CRON_INTERVAL_SECS")
                .unwrap_or_else(|_| "15".into())
                .parse()
                .unwrap_or(15);
            let message = std::env::var("CRON_MESSAGE")
                .unwrap_or_else(|_| "⏰ Default cron task running...".into());

            let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
            println!(
                "🕓 Cron started: every {}s | message='{}'",
                interval_secs, message
            );

            loop {
                ticker.tick().await;
                if cron_flag.load(Ordering::SeqCst) {
                    println!("🛑 Cron stopped due to shutdown.");
                    break;
                }

                // Submit background cron job to worker pool
                let msg = message.clone();
                let pool = pool_clone.clone();
                pool.submit(async move {
                    println!("⚙️ Running cron job: {}", msg);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    println!("✅ Completed cron job: {}", msg);
                })
                .await;
            }
        });
    }

    // --- Main server loop ---
    loop {
        tokio::select! {
            // Accept incoming connections
            Ok((mut socket, addr)) = listener.accept() => {
                if is_shutting_down.load(Ordering::SeqCst) {
                    println!("🛑 Server is shutting down. No new connections.");
                    break;
                }

                let middlewares = middlewares.clone();
                let db = db.clone();
                let cache_clone = cache.clone();
                let metrics = metrics.clone();
                let pool_clone = worker_pool.clone();
                let pubsub_manager = pubsub_manager.clone();
                let pubsub_handler = pubsub_handler.clone();

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

                        let pubsub_manager = pubsub_manager.clone();
                        let pubsub_handler = pubsub_handler.clone();
                        let handler: Box<
                            dyn Fn(&str) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync,
                        > = Box::new(move |req: &str| {
                            let req_owned = req.to_string();
                            let db = db.clone();
                            let cache = cache_for_handler.clone();
                            let metrics = metrics.clone();
                            let pool = pool_clone.clone();

                            let pubsub_manager = pubsub_manager.clone();
                            let pubsub_handler = pubsub_handler.clone();
                            Box::pin(async move {
                                route_request(&req_owned, &db, &cache, metrics, pool, &pubsub_handler).await
                            })
                        });

                        let res = middleware::run_chain(&req, &(addr.ip().to_string()), &middleware_ref, handler).await;
                        let _ = socket.write_all(res.into_http().as_bytes()).await;
                    }
                });
            }

            // Shutdown trigger
            _ = notify.notified() => {
                println!("🧩 Finishing in-flight requests...");
                break;
            }
        }
    }

    // --- Cleanup ---
    println!("✅ Graceful shutdown complete. Closing resources...");
    drop(listener);
    drop(worker_pool);
    cache.flush().ok();
    Ok(())
}

// --- Route Request Dispatcher ---
pub async fn route_request(
    req: &str,
    db: &Database,
    cache: &sled::Db,
    metrics: Arc<MetricsMiddleware>,
    worker_pool: Arc<WorkerPool>,
    pubsub_handler: &PubSubHandler,
) -> Response {
    if req.starts_with("GET /health") {
        handlers::health::handle().await
    } else if req.starts_with("GET /hello/") {
        handlers::hello::handle(req).await
    } else if req.starts_with("POST /user") {
        handlers::user::handle(req, db, cache).await
    } else if req.starts_with("GET /user") {
        handlers::user::get(req, db, cache).await
    } else if req.starts_with("POST /auth/token") {
        handlers::auth::create_token(req).await
    } else if req.starts_with("POST /auth/verify") {
        handlers::auth::verify_token(req).await
    } else if req.starts_with("GET /api/metrics") {
        metrics.handle_metrics()
    } else if req.starts_with("POST /publish") {
        return pubsub_handler.publish(req).await;
    } else if req.starts_with("POST /subscribe") {
        return pubsub_handler.subscribe(req).await;
    } else if req.starts_with("POST /jobs") {
        // Example: submit background job from HTTP route
        worker_pool
            .submit(async move {
                println!("🧠 Processing background job...");
                tokio::time::sleep(Duration::from_secs(3)).await;
                println!("✅ Job done");
            })
            .await;

        Response {
            status: 200,
            content_type: "text/plain".into(),
            body: "Background job submitted!".into(),
        }
    } else {
        Response {
            status: 404,
            content_type: "text/plain".into(),
            body: "Not Found".into(),
        }
    }
}
