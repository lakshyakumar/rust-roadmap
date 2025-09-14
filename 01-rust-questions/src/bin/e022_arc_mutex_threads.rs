// 22. How do you use Arc<Mutex<_>> to share a counter incremented by multiple threads? Use thread::scope to avoid 'static bounds.
// Explain the difference between Arc and Rc in multithreaded contexts.

use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..=5 {
        let counter_clone = Arc::clone(&counter);
        let handle = spawn(move || {
            let mut ctr = counter_clone.lock().unwrap();
            *ctr += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Final counter value after all thread processing is : {}",
        *counter.lock().unwrap()
    )
}
