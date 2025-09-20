// 53. What are the basics of unsafe in Rust?
// Create and dereference raw pointers, demonstrate undefined behavior risks, and show correct usage within an unsafe block.
// Why is unsafe needed?
fn main() {
    let mut x = 42;

    // Create raw pointers
    let ptr_const: *const i32 = &x;
    let ptr_mut: *mut i32 = &mut x;

    println!("Before unsafe: x = {}", x);

    // Dereferencing raw pointers requires `unsafe`
    unsafe {
        println!("ptr_const points to: {}", *ptr_const);

        *ptr_mut += 1; // mutate via raw pointer
        println!("ptr_mut updated value: {}", *ptr_mut);
    }

    println!("After unsafe: x = {}", x);

    // ---- Potential undefined behavior (UB) ----
    // Example: creating two mutable aliases
    let mut y = 100;
    let ptr1: *mut i32 = &mut y;
    let ptr2: *mut i32 = &mut y;

    unsafe {
        // This compiles, but is UB at runtime if both are used
        *ptr1 += 1;
        *ptr2 += 1; // ❌ UB: two mutable accesses to same memory
    }

    println!("y after UB (undefined behavior): {}", y);

    // ---- Correct usage: FFI or low-level systems ----
    let arr = [10, 20, 30];
    let ptr = arr.as_ptr();

    unsafe {
        // Safe if we respect array bounds
        for i in 0..arr.len() {
            println!("arr[{}] = {}", i, *ptr.add(i));
        }
    }
}
