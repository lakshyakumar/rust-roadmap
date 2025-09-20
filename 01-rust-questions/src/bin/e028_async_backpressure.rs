// 28. How do you implement backpressure in async Rust using tokio::sync::mpsc bounded channels?
// Why do producers need to await send()? Discuss the importance of backpressure in scalable systems.

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(2);

    // Producer
    tokio::spawn(async move {
        for i in 0..5 {
            println!("Sending {i}...");
            // Will await if buffer is full
            tx.send(i).await.unwrap();
            println!("Sent {i}");
        }
    });

    // Consumer (slower)
    while let Some(value) = rx.recv().await {
        println!("Received {value}");
        sleep(Duration::from_secs(2)).await; // simulate slow work
    }
}
