// 27. How do you use tokio::time::timeout for async requests?
// Handle Elapsed errors and cancel a background task by dropping its handle.
// Why is timeout handling critical in network programming?
use reqwest;
use tokio::time::{timeout, Duration};

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

#[tokio::main]
async fn main() {
    let url = "https://httpbin.org/delay/5"; // responds after 5s

    // Timeout set to 2 seconds
    match timeout(Duration::from_secs(3), fetch_url(url)).await {
        Ok(Ok(body)) => println!("Got response: {} chars", body.len()),
        Ok(Err(e)) => eprintln!("Request failed: {}", e),
        Err(_) => eprintln!("⚠️ Timeout: request took too long!"),
    }

    // doping handle for canceling task
    let handle = tokio::spawn(async {
        loop {
            println!("Background task running...");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // Allow it to run for 3s max
    if let Err(_) = timeout(Duration::from_secs(3), async {
        handle.await.ok();
    })
    .await
    {
        println!("Timeout reached, dropping task handle!");
        // handle is dropped here, cancelling the background task
        // (no need to explicitly call drop, just let it go out of scope)
    }
}
