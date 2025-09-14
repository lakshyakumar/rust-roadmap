// 15. Describe the drop order and RAII in Rust. Implement a TempDir that creates a temp folder in new() and removes it in Drop.
// How can you test drop order with print statements? Why is RAII important for resource management?
use std::env;
use std::fs;
use std::path::PathBuf;

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> Self {
        let mut path = env::temp_dir();
        path.push(name);

        fs::create_dir_all(&path).expect("Failed to create temp dir");
        println!("Created temp dir at {:?}", path);

        TempDir { path }
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).expect("Failed to remove temp dir");
            println!("Removed temp dir at {:?}", self.path);
        }
    }
}

fn main() {
    {
        let tmp1 = TempDir::new("example1");
        let tmp2 = TempDir::new("example2");

        println!("Temp dirs created");

        // tmp2 will be dropped before tmp1 (reverse order of creation)
    }
    println!("Scope ended, both TempDirs dropped");
}
