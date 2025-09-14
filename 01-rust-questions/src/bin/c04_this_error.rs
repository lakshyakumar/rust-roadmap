use std::fmt::{self, Display, Formatter};
use std::io;
use thiserror::Error;

#[derive(Debug)]
pub enum MyError {
    ServerError,
    VaidationError {
        field_name: String,
        failiure_str: String,
    },
    NetworkError(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyError::ServerError => write!(f, "Server error"),
            MyError::VaidationError {
                field_name,
                failiure_str,
            } => {
                write!(f, "Validation error on '{}': {}", field_name, failiure_str)
            }
            MyError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

#[derive(Error, Debug)]
pub enum ThisError {
    #[error("Internal Server Error")]
    ServerError,
    #[error("Validation Error: Field: {} Failiure: {}", .field_name, .failiure_str)]
    VaidationError {
        field_name: String,
        failiure_str: String,
    },
    #[error("Network error: {}", .0)]
    NetworkError(io::Error),
}

fn main() {
    println!("Server error {}", ThisError::ServerError);
    println!(
        "Validation error {}",
        ThisError::VaidationError {
            field_name: "name".to_string(),
            failiure_str: "too short".to_string()
        }
    );
    println!(
        "Network Error {}",
        ThisError::NetworkError(io::Error::new(
            io::ErrorKind::Other,
            "Check your connection"
        ))
    );
    println!(
        "Network Error {}",
        MyError::NetworkError(
            io::Error::new(std::io::ErrorKind::ConnectionAborted, "connection aborted").to_string()
        )
    );
}
