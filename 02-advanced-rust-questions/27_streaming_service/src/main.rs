use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
    time::{self, Duration, interval},
};

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tracing::{error, info};

#[derive(Clone)]
struct StreamingConfig {
    chunk_size: usize,
    chunk_interval: Duration,
    buffer_capacity: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Arc::new(StreamingConfig {
        chunk_size: 8 * 1024,
        chunk_interval: Duration::from_millis(300),
        buffer_capacity: 8,
    });
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("listener on ws://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let config = config.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, config).await {
                error!("connection error: {}", e)
            }
        });
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream, config: Arc<StreamingConfig>) -> anyhow::Result<()> {
    let ws_stream = accept_async(stream).await?;
    info!("new websocket client connected");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = mpsc::channel::<Bytes>(config.buffer_capacity);

    // Producer: generates fake binary data periodically
    let producer_config = config.clone();
    let producer_tx = tx.clone();
    let producer = tokio::spawn(async move {
        let mut ticker = interval(producer_config.chunk_interval);

        loop {
            ticker.tick().await;
            let chunk = vec![1u8; producer_config.chunk_size];
            if producer_tx.send(Bytes::from(chunk)).await.is_err() {
                // Channel closed (client likely gone)
                info!("producer exiting: receiver dropped");
                break;
            }
        }
    });

    // Consumer: sends chunks over websocket
    let sender_task = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            // Sending applies backpressure internally — if client is slow, this .await waits
            if let Err(e) = ws_sender.send(Message::Binary(bytes)).await {
                error!("send error: {}", e);
                break;
            }
        }
        info!("sender task done");
    });

    // Reader: handles incoming messages (like pings/close)
    let receiver_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(t)) => {
                    info!("client says: {}", t);
                    if t == "close" {
                        info!("client requested close");
                        break;
                    }
                }
                Ok(Message::Close(_)) => break,
                Ok(_) => {}
                Err(e) => {
                    error!("recv error: {}", e);
                    break;
                }
            }
        }
        info!("receiver task done");
    });

    // Wait for either task to end
    tokio::select! {
        _ = sender_task => {}
        _ = receiver_task => {}
    }
    producer.abort();
    drop(tx);
    producer.abort();

    info!("connection closed");
    Ok(())
}
