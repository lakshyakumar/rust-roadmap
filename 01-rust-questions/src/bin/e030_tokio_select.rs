// 30. How do you use tokio::select! to race a TCP read against a timeout, and handle whichever completes first?
// Why is select useful for multiplexing async events?

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Connect to server
    let mut stream = TcpStream::connect("example.com:80").await?;

    // Send a simple HTTP request
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await?;

    let mut buf = vec![0; 1024];

    // Race: read vs timeout
    tokio::select! {
        result = stream.read(&mut buf) => {
            match result {
                Ok(0) => println!("Connection closed by peer"),
                Ok(n) => println!("Read {} bytes", n),
                Err(e) => eprintln!("Read error: {}", e),
            }
        }
        _ = time::sleep(Duration::from_secs(3)) => {
            println!("⚠️ Timeout reached! Cancelling read...");
        }
    }

    Ok(())
}
