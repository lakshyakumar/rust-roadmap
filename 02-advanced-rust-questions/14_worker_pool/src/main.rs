mod cache;
mod handlers;
mod middleware;
mod middlewares;
mod server;
mod types;
mod workers;
mod pubsub;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::run().await?;
    Ok(())
}
