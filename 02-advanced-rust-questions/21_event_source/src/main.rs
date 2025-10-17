use std::env;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

mod event_store;
mod log_client;
mod logger;
mod order_server;
mod user_server;

async fn proxy_connection(mut inbound: TcpStream) {
    let mut buffer = [0; 1024];
    let n = inbound.read(&mut buffer).await.unwrap();
    let request = String::from_utf8_lossy(&buffer[..n]);

    // Determine backend
    let backend_addr = if request.starts_with("GET /users") {
        event_store::append_event("proxy", "GET /users").await;
        "127.0.0.1:3001"
    } else if request.starts_with("GET /orders") {
        event_store::append_event("proxy", "GET /orders").await;
        "127.0.0.1:3002"
    } else {
        inbound
            .write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found")
            .await
            .unwrap();
        return;
    };

    // Connect to backend
    let mut backend = TcpStream::connect(backend_addr).await.unwrap();
    backend.write_all(&buffer[..n]).await.unwrap();

    let mut backend_response = vec![0; 1024];
    let m = backend.read(&mut backend_response).await.unwrap();

    inbound.write_all(&backend_response[..m]).await.unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--rebuild" {
        event_store::rebuild_state();
        return;
    }
    // Spawn backend servers
    spawn(async { logger::run().await }); // Start the centralized logger
    log_client::send_log("proxy", "Reverse proxy started").await;
    spawn(async { user_server::run().await });
    spawn(async { order_server::run().await });

    // Start reverse proxy
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Reverse proxy running on 127.0.0.1:3000");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        spawn(proxy_connection(socket));
    }
}
