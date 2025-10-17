use crate::log_client;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:3002").await.unwrap();
    println!("Order server running on 127.0.0.1:3002");
    log_client::send_log("order_server", "Order server started").await;
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);

            let response = if request.starts_with("GET /orders") {
                log_client::send_log("order_server", "Received /orders request").await;
                let orders = json!([
                    { "id": 101, "item": "Book" },
                    { "id": 102, "item": "Laptop" }
                ]);
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    orders.to_string().len(),
                    orders
                )
            } else {
                "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found".to_string()
            };

            socket.write_all(response.as_bytes()).await.unwrap();
        });
    }
}
