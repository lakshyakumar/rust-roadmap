// 21. How do you implement a producer/consumer pattern using bounded channels (sync_channel) in Rust?
// Spawn threads and ensure graceful shutdown by dropping the sender. What are the challenges of concurrent programming in Rust?

use std::sync::mpsc;
use std::thread::spawn;

fn main() {
    let (tx, rx) = mpsc::channel();

    for _ in 1..=3 {
        let cloned_tx = tx.clone();
        spawn(move || {
            cloned_tx.send("Hi").unwrap();
        });
    }

    drop(tx);

    for received in rx {
        println!("Got: {}", received);
    }
}
