use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

struct Bucket {
    token: f64,
    last_refill: Instant,
}

impl Bucket {
    fn new(capacity: f64) -> Self {
        Self {
            token: capacity,
            last_refill: Instant::now(),
        }
    }

    fn allow(&mut self, capacity: f64, refill_rate: f64) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.last_refill = now;
        self.token = (self.token + elapsed * refill_rate).min(capacity);

        if self.token > 1.0 {
            self.token -= 1.0;
            true
        } else {
            false
        }
    }
}

pub struct RateLimiter {
    buckets: Mutex<HashMap<String, Bucket>>,
    capacity: f64,
    refill_rate: f64,
}

impl RateLimiter {
    pub fn new(capacity: f64, refill_rate: f64) -> Arc<Self> {
        Arc::new(Self {
            buckets: Mutex::new(HashMap::new()),
            capacity,
            refill_rate,
        })
    }

    pub async fn check(&self, ip: &str) -> bool {
        let mut buckets = self.buckets.lock().await;
        let bucket = buckets
            .entry(ip.to_string())
            .or_insert_with(|| Bucket::new(self.capacity));
        bucket.allow(self.capacity, self.refill_rate)
    }
}
