// 91. How do you add a Redis caching layer with TTL in Rust?
// Mitigate cache stampede with random jitter. What are the best practices for caching?
use rand::Rng;
use redis::{Commands, Connection, RedisResult};
use std::time::Duration;

/// Helper: connect to Redis
fn redis_connection() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}

/// Set a cache value with TTL + jitter
fn cache_set(key: &str, value: &str, ttl_secs: u64) -> RedisResult<()> {
    let mut con = redis_connection()?;

    // add random jitter between 0–10% of ttl
    let jitter: u64 = rand::thread_rng().gen_range(0..=(ttl_secs / 10).max(1));
    let ttl_with_jitter = ttl_secs + jitter;

    con.set_ex(key, value, ttl_with_jitter)?;
    Ok(())
}

/// Get a cached value if available
fn cache_get(key: &str) -> RedisResult<Option<String>> {
    let mut con = redis_connection()?;
    let result: Option<String> = con.get(key)?;
    Ok(result)
}

/// Example: fetch some data with caching
fn get_data_with_cache(key: &str) -> String {
    let ttl = 60; // 1 min TTL

    // 1. Try cache
    if let Ok(Some(value)) = cache_get(key) {
        println!("Cache hit for {}", key);
        return value;
    }

    // 2. Compute or fetch from DB/remote API
    println!("Cache miss for {}", key);
    let computed_value = format!("Expensive result at {:?}", std::time::SystemTime::now());

    // 3. Store in cache
    let _ = cache_set(key, &computed_value, ttl);

    computed_value
}

fn main() {
    // simulate calls
    let val1 = get_data_with_cache("item:123");
    println!("First call => {}", val1);

    // second call should be cached
    let val2 = get_data_with_cache("item:123");
    println!("Second call => {}", val2);
}
