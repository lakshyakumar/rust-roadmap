// Arc<T> → Atomic Reference Counting
// Stands for Atomic Reference Counted.
// Allows multiple immutable owners across threads.
// Only gives read-only access by default. (For mutation, you pair it with Mutex or RwLock.)

// Mutex<T> → Mutual Exclusion Lock
// Ensures only one mutable borrow at a time (just like borrow checker, but at runtime).
// lock() gives you a MutexGuard, which auto-releases when it goes out of scope.

// Arc<Mutex<T>> → Shared Mutable State Across Threads
// Arc → lets multiple threads share access.
// Mutex → ensures they don’t all write at once.

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // ARC example
    let data = Arc::new(42);
    let mut handles = vec![];

    for _ in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!(
                "thread sees value = {}, with reference count = {}",
                data_clone,
                Arc::strong_count(&data_clone)
            );
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!(
        "main sees value = {}, with reference count = {}",
        data,
        Arc::strong_count(&data)
    );

    // Mutex
    let data = Mutex::new(0);
    {
        let mut value = data.lock().unwrap(); // Lock acquired
        *value += 1; // Mutate safely
    }
    println!("Final value = {}", *data.lock().unwrap());

    // Arc<Mutex<T>> → Shared Mutable State Across Threads

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("final counter = {}", *counter.lock().unwrap());
}
