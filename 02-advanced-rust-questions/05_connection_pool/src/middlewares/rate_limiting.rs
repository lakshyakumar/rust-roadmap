use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

use crate::middleware::Middleware;
use crate::types::Response;

pub struct TokenBucketMiddleware {
    capacity: u32,
    refill_rate: u32,
    buckets: Mutex<HashMap<String, (u32, std::time::Instant)>>,
}

impl TokenBucketMiddleware {
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        TokenBucketMiddleware {
            capacity,
            refill_rate,
            buckets: Mutex::new(HashMap::new()),
        }
    }
}

impl Middleware for TokenBucketMiddleware {
    fn handle(
        &self,
        req: &str,
        client_ip: &str,
        next: &dyn for<'a> Fn(&'a str) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        let mut buckets = self.buckets.lock().unwrap();
        let (tokens, last) = buckets
            .entry(client_ip.to_string())
            .or_insert((self.capacity, std::time::Instant::now()));
        let elapsed = last.elapsed().as_secs_f64();
        let refill_token = (elapsed * self.refill_rate as f64) as u32;
        *tokens = (*tokens + refill_token).min(self.capacity);
        *last = std::time::Instant::now();

        if *tokens == 0 {
            let response = Response {
                status: 429,
                content_type: "text/plain".into(),
                body: "Too Many Requests".into(),
            };
            return Box::pin(async move { response });
        }
        *tokens -= 1;
        next(req)
    }
}
