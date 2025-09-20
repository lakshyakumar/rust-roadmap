// 29. How do you expose a tokio::sync::mpsc channel as a Stream using tokio_stream::wrappers::ReceiverStream?
// Consume the stream with while let Some(x). What are the benefits of using streams in async Rust?

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(5);

    // Spawn a producer
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    // Convert receiver into a Stream
    let mut stream = ReceiverStream::new(rx);

    // Consume it with while let Some(x)
    while let Some(value) = stream.next().await {
        println!("Got: {value}");
    }

    println!("Stream finished.");
}
