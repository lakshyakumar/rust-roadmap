use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:4000").await.unwrap();
    println!("Log Aggregator running on 127.0.0.1:4000");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => break, // connection closed
                    Ok(n) => {
                        let log_entry = String::from_utf8_lossy(&buffer[..n]);
                        println!("[LOG][{}]: {}", addr, log_entry.trim());
                    }
                    Err(_) => break,
                }
            }
        });
    }
}
