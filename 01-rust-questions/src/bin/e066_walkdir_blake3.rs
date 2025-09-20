// 66. How do you walk the filesystem using walkdir to compute blake3 hashes of files concurrently?
// Show the first 8 hex characters of each hash. What are the challenges of concurrent file I/O?
use blake3::Hasher;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use walkdir::WalkDir;

fn main() {
    let root = "."; // start directory

    // Collect file paths first
    let files: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    // Process in parallel with Rayon
    files.par_iter().for_each(|entry| {
        let path = entry.path();
        match File::open(path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut hasher = Hasher::new();
                let mut buffer = [0u8; 8192];

                while let Ok(n) = reader.read(&mut buffer) {
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }

                let hash = hasher.finalize();
                // show first 8 hex chars
                println!("{:8}  {}", &hash.to_hex()[..8], path.display());
            }
            Err(err) => {
                eprintln!("Failed to read {}: {}", path.display(), err);
            }
        }
    });
}
