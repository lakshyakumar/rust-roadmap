// 57. What is pinning in Rust?
// Implement a self-referential struct using Pin<Box<T>> with a field holding a pointer into itself, and explain why pinning is needed.
// How does pinning prevent memory movement?

// In Rust, normally you cannot safely create a struct that holds a pointer/reference to itself, because moving the struct would invalidate the pointer.
// Example (invalid!):
// struct SelfRef<'a> {
//     value: String,
//     ptr: *const String, // points inside `value`
// }
// If the struct moves in memory, ptr dangles → undefined behavior.
// 🔹 Solution: Pinning
// Pin<Box<T>> guarantees that once allocated, the value will not move in memory.
// That lets us safely hold pointers into our own fields.

use std::pin::Pin;

struct SelfRef {
    value: String,
    ptr: *const String,
}

impl SelfRef {
    fn new(txt: &str) -> Pin<Box<SelfRef>> {
        // Step 1: Create with dummy pointer
        let mut s = Box::pin(SelfRef {
            value: txt.to_string(),
            ptr: std::ptr::null(),
        });

        // Step 2: Get a stable pointer to `value`
        let ptr: *const String = &s.value;

        // Step 3: Update self-referential pointer
        // SAFETY: We're inside Pin, so the struct will not move.
        let mut_ref = unsafe { Pin::as_mut(&mut s) };
        unsafe {
            Pin::get_unchecked_mut(mut_ref).ptr = ptr;
        }

        s
    }

    fn get_ref(&self) -> &String {
        assert!(!self.ptr.is_null());
        unsafe { &*self.ptr }
    }
}

fn main() {
    let s = SelfRef::new("Hello, pinning!");

    println!("Original value: {}", s.value);
    println!("Via self pointer: {}", s.get_ref());
}
