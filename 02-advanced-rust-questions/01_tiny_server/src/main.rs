use anyhow::Ok;
use std::collections::HashMap;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod routes;
mod utils;

mod rate_limiter;
use rate_limiter::RateLimiter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    println!("Listening on 0.0.0.0:7878");

    // Allow burst of 5 requests, refill 1 request per second
    let limiter = RateLimiter::new(5.0, 1.0);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepted {}", addr);
        let limiter = limiter.clone();
        let ip = addr.ip().to_string();

        tokio::spawn(async move {
            if !limiter.check(&ip).await {
                let _ = socket
                    .write_all(b"HTTP/1.1 429 Too Many Requests\r\n\r\nRate limit exceeded")
                    .await;
                println!("Throttled request from {}", ip);
                return;
            }
            if let Err(e) = handle_connection(&mut socket).await {
                eprintln!("connection Error ({}): {}", addr, e);
                let _ = socket.shutdown().await;
            }
        });
    }
}

async fn handle_connection(stream: &mut tokio::net::TcpStream) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 1024];

    let header_end_pos = loop {
        let n = stream.read(&mut tmp).await?;
        if n == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = utils::find_header_end(&buf) {
            break pos;
        }
        if buf.len() > 64 * 1024 {
            return Err(anyhow::anyhow!("headers too large"));
        }
    };

    let headers_bytes = &buf[..header_end_pos];
    let headers_str = str::from_utf8(headers_bytes)?;
    // println!("Header String: {}", &headers_str);
    let mut lines = headers_str.split("\r\n");

    let request_line = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing request line"))?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let path = parts.next().unwrap_or_default().to_string();

    let mut header_map = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((k, v)) = utils::split_once(line, ':') {
            header_map.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }

    let body_already = buf.len() - (header_end_pos + 4);
    let content_length: usize = header_map
        .get("content-length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let mut body = Vec::with_capacity(content_length);
    if body_already > 0 {
        let start = header_end_pos + 4;
        let avail = buf.len() - start;
        let to_take = avail.min(content_length);
        body.extend_from_slice(&buf[start..start + to_take]);
    }
    while body.len() < content_length {
        let n = stream.read(&mut tmp).await?;
        if n == 0 {
            break;
        }
        let need = content_length - body.len();
        let take = n.min(need);
        body.extend_from_slice(&tmp[..take]);
    }
    let body_text = String::from_utf8_lossy(&body).to_string();
    // println!("Body Text: \n{}\n", &body_text);
    let response = routes::router(&method, &path, &header_map, &body_text);

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}
