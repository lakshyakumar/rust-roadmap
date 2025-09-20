// 58. How do you use uninitialized memory safely in Rust?
// Use MaybeUninit<[u8; 1024]> to fill a buffer from a read operation, avoid UB, and then call assume_init().
// What are the dangers of uninitialized memory?

// Rust normally forbids uninitialized memory because reading it is undefined behavior (UB).
// Sometimes you want uninitialized memory for performance (e.g., buffers).
// MaybeUninit<T> lets you safely represent “a T that might not be initialized yet.”

use std::io::{self, Read};
use std::mem::MaybeUninit;

fn main() -> io::Result<()> {
    // Step 1: Allocate uninitialized buffer
    let mut buf: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Step 2: Get a raw pointer to the buffer
    let buf_ptr = buf.as_mut_ptr();

    // Step 3: SAFETY
    // - buf_ptr is valid for 1024 bytes
    // - We won't read uninitialized parts
    let initialized: usize = unsafe {
        let slice = &mut *buf_ptr;
        handle.read(slice)? // Fills some prefix of the buffer
    };

    // Step 4: Mark buffer as fully initialized (dangerous!)
    // Actually only first `initialized` bytes are valid.
    let buf = unsafe { buf.assume_init() };

    println!("Read {} bytes", initialized);
    println!("First few: {:?}", &buf[..initialized.min(10)]);

    Ok(())
}
