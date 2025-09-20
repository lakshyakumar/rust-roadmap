// 36. How do you use thiserror for precise library errors and anyhow for error bubbling in a binary?
// Show how to add context to errors. Why is error context important?

use anyhow::{Context, Result};
use std::fs;
use std::io::Read;
use thiserror::Error;

// Library error type using thiserror
#[derive(Debug, Error)]
pub enum MyLibError {
    #[error("Error opening file: {0}")]
    FileOpen(#[from] std::io::Error),
    #[error("Data empty in file '{0}'")]
    EmptyData(String),
}

// Library function returning precise errors
fn read_data(path: &str) -> std::result::Result<String, MyLibError> {
    let mut file = fs::File::open(path).map_err(MyLibError::FileOpen)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(MyLibError::FileOpen)?;
    if contents.trim().is_empty() {
        Err(MyLibError::EmptyData(path.to_string()))
    } else {
        Ok(contents)
    }
}

// Application/main using anyhow for error bubbling and context
fn main() -> Result<()> {
    // Try reading a file, bubbling up and adding context at each step
    let result = read_data("example.txt").with_context(|| "Failed to read example.txt")?;

    println!("File contents: {}", result);
    Ok(())
}
