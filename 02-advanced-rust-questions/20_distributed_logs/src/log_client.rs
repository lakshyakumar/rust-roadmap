use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn send_log(service: &str, message: &str) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:4000").await {
        let log = format!(r#"{{"service": "{}", "message": "{}"}}"#, service, message);
        let _ = stream.write_all(log.as_bytes()).await;
    }
}
