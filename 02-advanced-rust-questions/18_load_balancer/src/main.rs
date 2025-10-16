use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

#[derive(Clone)]
struct BackendPool {
    backends: Vec<String>,
    next: Arc<Mutex<usize>>,
}

impl BackendPool {
    fn new(backends: Vec<&str>) -> Self {
        Self {
            backends: backends.into_iter().map(|s| s.to_string()).collect(),
            next: Arc::new(Mutex::new(0)),
        }
    }

    async fn get_next_backend(&self) -> String {
        let mut idx = self.next.lock().await;
        let backend = self.backends[*idx].clone();
        *idx = (*idx + 1) % self.backends.len();
        backend
    }
}

async fn handle_client(mut inbound: TcpStream, pool: BackendPool) {
    let mut buffer = [0; 4096];
    let n = match inbound.read(&mut buffer).await {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let backend_addr = pool.get_next_backend().await;

    if let Ok(mut backend) = TcpStream::connect(backend_addr).await {
        // Forward request
        backend.write_all(&buffer[..n]).await.unwrap();

        // Read backend response
        let mut resp = vec![0; 4096];
        let m = backend.read(&mut resp).await.unwrap();

        // Send back to client
        inbound.write_all(&resp[..m]).await.unwrap();
    } else {
        inbound
            .write_all(b"HTTP/1.1 502 BAD GATEWAY\r\n\r\nBackend not reachable")
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    // Example backend servers (replace with multiple backends if needed)
    let backend_pool = BackendPool::new(vec!["127.0.0.1:3001", "127.0.0.1:3002"]);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Load balancer running on 127.0.0.1:3000");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let pool = backend_pool.clone();
        tokio::spawn(handle_client(socket, pool));
    }
}
