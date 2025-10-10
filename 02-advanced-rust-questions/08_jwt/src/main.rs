mod cache;
mod handlers;
mod middleware;
mod middlewares;
mod server;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::run().await?;
    Ok(())
}
