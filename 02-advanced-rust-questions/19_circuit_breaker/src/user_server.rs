use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("User server running on 127.0.0.1:3001");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.unwrap();
            let request = String::from_utf8_lossy(&buffer[..n]);

            let response = if request.starts_with("GET /users") {
                let users = json!([
                    { "id": 1, "name": "Alice" },
                    { "id": 2, "name": "Bob" }
                ]);
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    users.to_string().len(),
                    users
                )
            } else {
                "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found".to_string()
            };

            socket.write_all(response.as_bytes()).await.unwrap();
        });
    }
}
