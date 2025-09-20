// 82. How do you build a bounded async queue with backpressure in Rust?
// Measure queue depth over time as producers and consumers operate at different speeds.
// Why is backpressure critical for system stability?
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(5); // bounded queue of size 5
    let queue_depth = Arc::new(Mutex::new(Vec::new()));

    // Producer: generates items quickly
    let prod_depth = queue_depth.clone();
    let producer = tokio::spawn(async move {
        for i in 0..20 {
            // Send will wait if the queue is full (backpressure!)
            tx.send(i).await.unwrap();
            {
                let mut depths = prod_depth.lock().unwrap();
                depths.push(("producer", i));
            }
            println!("Produced {i}");
            sleep(Duration::from_millis(100)).await; // producing faster
        }
    });

    // Consumer: processes items slowly
    let cons_depth = queue_depth.clone();
    let consumer = tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            println!("Consumed {item}");
            {
                let mut depths = cons_depth.lock().unwrap();
                depths.push(("consumer", item));
            }
            sleep(Duration::from_millis(300)).await; // consuming slower
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();

    // Print queue depth timeline
    let depths = queue_depth.lock().unwrap();
    println!("\nTimeline of events (producer vs consumer):");
    for (actor, value) in depths.iter() {
        println!("{actor} -> {value}");
    }
}
