// 23. How do you implement a read-mostly cache using RwLock<HashMap<K,V>>?\
// Measure lock time with Instant and explain lock contention. What are the benefits and drawbacks of read-write locks?

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

struct InstrumentedCache {
    map: RwLock<HashMap<String, i32>>,

    // accumulated nanoseconds counters
    read_wait_ns: AtomicU64,
    read_hold_ns: AtomicU64,
    write_wait_ns: AtomicU64,
    write_hold_ns: AtomicU64,
}

#[derive(Debug)]
struct CacheStats {
    read_wait_ns: u64,
    read_hold_ns: u64,
    write_wait_ns: u64,
    write_hold_ns: u64,
}

impl InstrumentedCache {
    fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
            read_wait_ns: AtomicU64::new(0),
            read_hold_ns: AtomicU64::new(0),
            write_wait_ns: AtomicU64::new(0),
            write_hold_ns: AtomicU64::new(0),
        }
    }

    fn get(&self, key: &str) -> Option<i32> {
        // Measure wait time until we acquire the read lock
        let t_wait_start = Instant::now();
        let guard = self.map.read().unwrap();
        let wait_dur = t_wait_start.elapsed();
        self.read_wait_ns
            .fetch_add(wait_dur.as_nanos() as u64, Ordering::Relaxed);

        // Measure how long we hold the lock for actual access
        let t_hold_start = Instant::now();
        let value = guard.get(key).cloned();
        let hold_dur = t_hold_start.elapsed();
        self.read_hold_ns
            .fetch_add(hold_dur.as_nanos() as u64, Ordering::Relaxed);

        value
    }

    /// Insert/overwrite a value.
    fn insert(&self, key: String, value: i32) {
        let t_wait_start = Instant::now();
        let mut guard = self.map.write().unwrap();
        let wait_dur = t_wait_start.elapsed();
        self.write_wait_ns
            .fetch_add(wait_dur.as_nanos() as u64, Ordering::Relaxed);

        let t_hold_start = Instant::now();
        guard.insert(key, value);
        // simulate a bit of work while holding the lock (optional)
        // thread::sleep(Duration::from_micros(50));
        let hold_dur = t_hold_start.elapsed();
        self.write_hold_ns
            .fetch_add(hold_dur.as_nanos() as u64, Ordering::Relaxed);
    }

    fn stats(&self) -> CacheStats {
        CacheStats {
            read_wait_ns: self.read_wait_ns.load(Ordering::Relaxed),
            read_hold_ns: self.read_hold_ns.load(Ordering::Relaxed),
            write_wait_ns: self.write_wait_ns.load(Ordering::Relaxed),
            write_hold_ns: self.write_hold_ns.load(Ordering::Relaxed),
        }
    }
}

fn main() {
    let cache = Arc::new(InstrumentedCache::new());

    // preload with some values
    for i in 0..100 {
        cache.insert(format!("k{}", i), i);
    }

    // spawn many reader threads and a few writers
    let mut handles = Vec::new();
    let readers = 8;
    let writers = 2;
    let reads_per_thread = 10_000;
    let writes_per_thread = 200;

    for ri in 0..readers {
        let c = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for i in 0..reads_per_thread {
                // choose a random-ish key
                let key = format!("k{}", (i + ri) % 150);
                let _ = c.get(&key);
                // simulate some processing time outside locks
                // thread::sleep(Duration::from_micros(5));
            }
        }));
    }

    // Writers (few)
    for wi in 0..writers {
        let c = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for i in 0..writes_per_thread {
                let key = format!("k{}", (i + wi * 100) % 200);
                c.insert(key, i as i32);
                // simulate some work outside locks
                // thread::sleep(Duration::from_micros(50));
            }
        }));
    }

    // wait for all threads
    for h in handles {
        h.join().unwrap();
    }

    // Print stats
    let s = cache.stats();
    println!("Cache stats (nanoseconds): {:?}", s);

    let total_reads = (readers * reads_per_thread) as u64;
    let total_writes = (writers * writes_per_thread) as u64;

    println!("Total read ops   : {}", total_reads);
    println!("Total write ops  : {}", total_writes);

    if total_reads > 0 {
        println!(
            "Avg read wait    : {:.3} µs",
            s.read_wait_ns as f64 / total_reads as f64 / 1_000.0
        );
        println!(
            "Avg read hold    : {:.3} µs",
            s.read_hold_ns as f64 / total_reads as f64 / 1_000.0
        );
    }
    if total_writes > 0 {
        println!(
            "Avg write wait   : {:.3} µs",
            s.write_wait_ns as f64 / total_writes as f64 / 1_000.0
        );
        println!(
            "Avg write hold   : {:.3} µs",
            s.write_hold_ns as f64 / total_writes as f64 / 1_000.0
        );
    }
}
