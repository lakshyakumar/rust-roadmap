// 37. How do you add spans and instrumentation to async handlers using the tracing crate?
// Include span fields and use the tracing-subscriber fmt layer.
// How does tracing help with observability?
use tokio::time::{sleep, Duration};
use tracing::{info, info_span, Instrument};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// Async handler instrumented with a span including fields
async fn handle_request(user_id: u64) {
    // Create a span with fields
    let span = info_span!("handle_request", user_id = user_id);

    // Instrument async block (or function call) with the span
    async {
        info!("Handling request start");
        sleep(Duration::from_secs(1)).await;
        info!("Handling request end");
    }
    .instrument(span)
    .await;
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with env filter and fmt layer for console output
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("info")) // Show at least info-level logs and spans
        .init();

    // Call instrumented async handler
    handle_request(42).await;
}
