// 31. How do you read a large file asynchronously using tokio::fs and compute its SHA-256 incrementally with tokio::io::BufReader?
// Discuss the challenges of async file I/O.

use sha2::{Digest, Sha256}; // cargo add sha2
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, BufReader};

#[tokio::main]
async fn main() -> io::Result<()> {
    let file = File::open("rust_interview_questions.md").await?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8 * 1024]; // 8 KB buffer

    // Read file in chunks asynchronously
    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break; // EOF
        }
        hasher.update(&buffer[..n]); // Incremental hashing
    }

    let result = hasher.finalize();
    println!("SHA-256: {:x}", result);

    Ok(())
}
