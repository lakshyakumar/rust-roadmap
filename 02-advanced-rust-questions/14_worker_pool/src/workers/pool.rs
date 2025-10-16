use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub type Job = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

#[derive(Clone)]
pub struct WorkerPool {
    sender: Sender<Job>,
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel(100);
        let rx = Arc::new(tokio::sync::Mutex::new(rx));

        for i in 0..size {
            let rx = rx.clone();
            tokio::spawn(async move {
                println!("🧵 Worker {} started", i);
                loop {
                    let job_opt = {
                        let mut guard = rx.lock().await;
                        guard.recv().await
                    };
                    match job_opt {
                        Some(job) => job.await,
                        None => {
                            println!("💤 Worker {} shutting down", i);
                            break;
                        }
                    }
                }
            });
        }
        Self { sender: tx }
    }

    pub async fn submit<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        if let Err(_) = self.sender.send(Box::pin(fut)).await {
            eprintln!("⚠️ Worker pool queue closed");
        }
    }
}
