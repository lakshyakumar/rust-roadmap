// 61. How do you replace std::sync locks with parking_lot locks in Rust?
// Benchmark the difference using the criterion crate. Why might you prefer parking_lot?

// parking_lot::Mutex and RwLock
// Drop-in replacements for std::sync versions.
// Implemented fully in user space using the parking lot algorithm (threads "park" instead of spin or busy wait).
// Faster lock/unlock, smaller memory footprint, more features (try_lock_timeout, fair/unfair locking).
// Widely used in performance-sensitive projects (e.g., tokio, rayon).

use criterion::{criterion_group, criterion_main, Criterion};
use parking_lot::Mutex as ParkingMutex;
use std::sync::{Arc, Mutex as StdMutex};

// Worker function that increments inside a lock
fn work_std(m: Arc<StdMutex<u64>>) {
    for _ in 0..1000 {
        let mut guard = m.lock().unwrap();
        *guard += 1;
    }
}

fn work_parking(m: Arc<ParkingMutex<u64>>) {
    for _ in 0..1000 {
        let mut guard = m.lock();
        *guard += 1;
    }
}

fn bench_mutexes(c: &mut Criterion) {
    c.bench_function("std::sync::Mutex", |b| {
        b.iter(|| {
            let m = Arc::new(StdMutex::new(0));
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let m = m.clone();
                    std::thread::spawn(move || work_std(m))
                })
                .collect();
            for h in handles {
                h.join().unwrap();
            }
        })
    });

    c.bench_function("parking_lot::Mutex", |b| {
        b.iter(|| {
            let m = Arc::new(ParkingMutex::new(0));
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let m = m.clone();
                    std::thread::spawn(move || work_parking(m))
                })
                .collect();
            for h in handles {
                h.join().unwrap();
            }
        })
    });
}

criterion_group!(benches, bench_mutexes);
criterion_main!(benches);
