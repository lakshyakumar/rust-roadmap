// 85. How do you implement a mini-Redis server that parses RESP and supports a subset of commands (PING, GET, SET) over TCP, handling concurrency?
// What are the challenges of building network servers?

// mini Redis server in Rust that speaks the RESP protocol and supports a few commands (PING, SET, GET). This will look like a toy Redis clone.

// 🔹 What’s RESP?
// Redis uses the REdis Serialization Protocol (RESP). It’s simple and text-based:
//    - Simple string: +OK\r\n
//    - Bulk string: $3\r\nfoo\r\n
//    - Array: *2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n

// Example:
// *2\r\n$4\r\nPING\r\n$4\r\nTEST\r\n

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, String>>>;

/// Parse a RESP command into a vector of strings
async fn parse_resp(stream: &mut (impl AsyncBufReadExt + Unpin)) -> Option<Vec<String>> {
    let mut line = String::new();
    if stream.read_line(&mut line).await.ok()? == 0 {
        return None;
    }

    if !line.starts_with('*') {
        return None; // Expecting array
    }
    let n: usize = line[1..].trim().parse().ok()?;
    let mut parts = Vec::with_capacity(n);

    for _ in 0..n {
        line.clear();
        stream.read_line(&mut line).await.ok()?; // bulk string header ($len)
        if !line.starts_with('$') {
            return None;
        }
        let len: usize = line[1..].trim().parse().ok()?;

        let mut buf = vec![0; len + 2]; // include \r\n
        stream.read_exact(&mut buf).await.ok()?;
        let s = String::from_utf8_lossy(&buf[..len]).to_string();
        parts.push(s);
    }

    Some(parts)
}

/// Handle a single TCP client
async fn handle_client(mut socket: TcpStream, db: Db) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    while let Some(cmd) = parse_resp(&mut reader).await {
        let response = match cmd[0].to_uppercase().as_str() {
            "PING" => {
                if cmd.len() > 1 {
                    format!("+{}\r\n", cmd[1])
                } else {
                    "+PONG\r\n".to_string()
                }
            }
            "SET" if cmd.len() == 3 => {
                let mut store = db.lock().unwrap();
                store.insert(cmd[1].clone(), cmd[2].clone());
                "+OK\r\n".to_string()
            }
            "GET" if cmd.len() == 2 => {
                let store = db.lock().unwrap();
                match store.get(&cmd[1]) {
                    Some(val) => format!("${}\r\n{}\r\n", val.len(), val),
                    None => "$-1\r\n".to_string(), // Null bulk string
                }
            }
            _ => "-ERR unknown command\r\n".to_string(),
        };

        if writer.write_all(response.as_bytes()).await.is_err() {
            break;
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    println!("Mini Redis running on 127.0.0.1:6379");

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move {
            handle_client(socket, db).await;
        });
    }
}
