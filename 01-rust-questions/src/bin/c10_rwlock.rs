// RwLock<T> = Read–Write Lock.
// Multiple readers (read() locks) OR one writer (write() lock).
// Prevents data races while improving performance compared to Mutex (which always blocks everyone else, even readers).

use std::sync::{Arc, RwLock};
use std::thread;

fn single_thread() {
    let data = RwLock::new(5);

    {
        let r1 = data.read().unwrap();
        let r2 = data.read().unwrap();
        println!("Readers: {}, {}", *r1, *r2);
    }

    {
        let mut w = data.write().unwrap();
        *w += 10;
        println!("Writer updated the value = {}", *w);
    }

    println!("Final value = {}", *data.read().unwrap());
}

fn multi_thread() {
    let shared = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let mut w = shared_clone.write().unwrap();
            *w += 1;
            println!("Writer updated to {}", *w);
        }));
    }

    for _ in 0..3 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let r = shared_clone.read().unwrap();
            println!("Reader sees {}", *r);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn main() {
    // single_thread();
    multi_thread();
}
