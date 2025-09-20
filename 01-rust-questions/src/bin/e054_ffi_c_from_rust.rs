// 54. How do you call C functions from Rust using FFI (Foreign Function Interface) ?
// Declare an extern "C" function to call puts from libc, link, and call it safely.
// What are the safety concerns with FFI?
use std::ffi::CString;

extern "C" {
    // Declaration of the C function `puts` from libc
    fn puts(s: *const i8) -> i32;
}

fn main() {
    // Rust string → C string (null-terminated)
    let msg = CString::new("Hello from Rust via C puts!").unwrap();

    unsafe {
        // Call the C function
        puts(msg.as_ptr());
    }

    println!("Back in Rust!");
}
