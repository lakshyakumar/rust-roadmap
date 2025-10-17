use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::{sync::Arc, time::{Duration, Instant}};

#[derive(Clone)]
struct Backend {
    address: String,
    failure_count: Arc<Mutex<u32>>,
    last_failure: Arc<Mutex<Option<Instant>>>,
    state: Arc<Mutex<CircuitState>>,
}

#[derive(Clone, Copy, PartialEq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Clone)]
struct BackendPool {
    backends: Vec<Backend>,
    next: Arc<Mutex<usize>>,
}

impl BackendPool {
    fn new(addrs: Vec<&str>) -> Self {
        Self {
            backends: addrs
                .into_iter()
                .map(|a| Backend {
                    address: a.to_string(),
                    failure_count: Arc::new(Mutex::new(0)),
                    last_failure: Arc::new(Mutex::new(None)),
                    state: Arc::new(Mutex::new(CircuitState::Closed)),
                })
                .collect(),
            next: Arc::new(Mutex::new(0)),
        }
    }

    async fn get_next_backend(&self) -> Option<Backend> {
        let total = self.backends.len();
        for _ in 0..total {
            let mut idx = self.next.lock().await;
            let backend = &self.backends[*idx];
            *idx = (*idx + 1) % total;

            if backend.is_available().await {
                return Some(backend.clone());
            }
        }
        None
    }
}

impl Backend {
    async fn is_available(&self) -> bool {
        let mut state = self.state.lock().await;
        match *state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                let last = self.last_failure.lock().await;
                if let Some(time) = *last {
                    if time.elapsed() > Duration::from_secs(10) {
                        *state = CircuitState::HalfOpen;
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    async fn record_success(&self) {
        let mut failures = self.failure_count.lock().await;
        *failures = 0;
        let mut state = self.state.lock().await;
        *state = CircuitState::Closed;
    }

    async fn record_failure(&self) {
        let mut failures = self.failure_count.lock().await;
        *failures += 1;

        if *failures >= 3 {
            let mut state = self.state.lock().await;
            *state = CircuitState::Open;
            let mut last = self.last_failure.lock().await;
            *last = Some(Instant::now());
            println!("⚠️ Backend {} circuit opened", self.address);
        }
    }
}

async fn handle_client(mut inbound: TcpStream, pool: BackendPool) {
    let mut buffer = [0; 4096];
    let n = match inbound.read(&mut buffer).await {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    if let Some(backend) = pool.get_next_backend().await {
        match TcpStream::connect(&backend.address).await {
            Ok(mut backend_stream) => {
                if backend_stream.write_all(&buffer[..n]).await.is_ok() {
                    let mut resp = vec![0; 4096];
                    match backend_stream.read(&mut resp).await {
                        Ok(m) if m > 0 => {
                            backend.record_success().await;
                            inbound.write_all(&resp[..m]).await.unwrap();
                            return;
                        }
                        _ => backend.record_failure().await,
                    }
                } else {
                    backend.record_failure().await;
                }
            }
            Err(_) => backend.record_failure().await,
        }
    }

    inbound
        .write_all(b"HTTP/1.1 502 BAD GATEWAY\r\n\r\nAll backends unavailable")
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let pool = BackendPool::new(vec!["127.0.0.1:3001", "127.0.0.1:3002"]);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Load Balancer with Circuit Breaker running on 127.0.0.1:3000");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let pool = pool.clone();
        tokio::spawn(handle_client(socket, pool));
    }
}