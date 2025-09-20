// 33. How do you define async trait objects using the async-trait crate?
// Implement a Repository trait with async fn get(id)->Result<Item> and provide an implementation. What are the limitations of async traits in Rust?

use async_trait::async_trait;

#[derive(Debug)]
struct Item {
    id: u32,
    name: String,
}

// Define an async trait
#[async_trait]
trait Repository {
    async fn get(&self, id: u32) -> Result<Item, String>;
}

// Provide an implementation
struct InMemoryRepo;

#[async_trait]
impl Repository for InMemoryRepo {
    async fn get(&self, id: u32) -> Result<Item, String> {
        // simulate async work
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Ok(Item {
            id,
            name: format!("Item-{}", id),
        })
    }
}

#[tokio::main]
async fn main() {
    let repo = InMemoryRepo;

    match repo.get(42).await {
        Ok(item) => println!("Got: {:?}", item),
        Err(e) => eprintln!("Error: {}", e),
    }
}
