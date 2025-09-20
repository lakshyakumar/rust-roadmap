// 26. What is the purpose of the #[tokio::main] attribute?
// Write an async main function that fetches two URLs concurrently using join!. How does async/await improve concurrency?

use tokio::join;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let url1 = "https://www.rust-lang.org";
    let url2 = "https://www.wikipedia.org";

    let (res1, res2) = join!(fetch_url(url1), fetch_url(url2));

    println!("Rust site length {}", res1.unwrap().len());
    println!("Wiki site length {}", res2.unwrap().len());
}
