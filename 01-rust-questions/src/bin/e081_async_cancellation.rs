// 81. How do you use tokio_util::sync::CancellationToken to cancel a long-running async task on Ctrl+C?
// What are the best practices for cancellation in async Rust?
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let child_token = token.child_token();

    // Spawn a long-running task
    let handle = tokio::spawn(long_running_task(child_token));

    // Listen for Ctrl+C
    tokio::spawn({
        let token = token.clone();
        async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for ctrl-c");
            println!("Ctrl+C received, cancelling...");
            token.cancel();
        }
    });

    // Wait for task
    if let Err(e) = handle.await {
        eprintln!("Task failed: {:?}", e);
    }

    println!("Shutdown complete.");
}

async fn long_running_task(token: CancellationToken) {
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("Task cancelled, cleaning up...");
                break;
            }
            _ = sleep(Duration::from_secs(1)) => {
                println!("Working...");
            }
        }
    }
}
