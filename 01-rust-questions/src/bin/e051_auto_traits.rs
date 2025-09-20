// 51. What are auto traits like Send and Sync?
// Build a type that is !Send due to a raw pointer, then fix it by wrapping in Arc<Mutex<>> and explain the results.
// How do auto traits affect concurrency?

use std::sync::{Arc, Mutex};
use std::thread;

// A type holding a raw pointer (not thread-safe).
struct NotSend {
    ptr: *mut i32,
}

// A safe wrapper: using Arc<Mutex<T>> makes it Send + Sync.
struct SafeSend {
    data: Arc<Mutex<i32>>,
}

fn main() {
    // Example 1: !Send type (raw pointer)
    let mut value = 10;
    let not_send = NotSend { ptr: &mut value };

    // Try to move `not_send` into a thread
    // let handle = thread::spawn(move || {
    //     unsafe {
    //         *not_send.ptr += 1;
    //     }
    // });
    //
    // handle.join().unwrap();
    //
    // ❌ ERROR: `NotSend` cannot be sent between threads safely
    println!("NotSend is !Send because raw pointers are not thread-safe.");

    // Example 2: SafeSend using Arc<Mutex<T>>
    let safe = SafeSend {
        data: Arc::new(Mutex::new(42)),
    };

    let safe_clone = SafeSend {
        data: Arc::clone(&safe.data),
    };

    let handle = thread::spawn(move || {
        let mut lock = safe_clone.data.lock().unwrap();
        *lock += 1;
        println!("Thread updated value to {}", *lock);
    });

    handle.join().unwrap();

    // Access from main thread
    let lock = safe.data.lock().unwrap();
    println!("Final value: {}", *lock);
}
