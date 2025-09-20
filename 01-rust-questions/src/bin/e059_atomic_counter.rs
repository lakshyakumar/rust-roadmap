// 59. How do you implement a lock-free concurrent counter using AtomicU64 and fetch_add?
// Discuss memory ordering (SeqCst vs Relaxed). Why is lock-free programming challenging?
// 59. How do you implement a lock-free concurrent counter using AtomicU64 and fetch_add?
// Discuss memory ordering (SeqCst vs Relaxed). Why is lock-free programming challenging?

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));

    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1_000_000 {
                // Increment counter atomically
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.load(Ordering::SeqCst));
}
