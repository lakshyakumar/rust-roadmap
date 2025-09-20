// 68. How do you profile memory usage in Rust?
// Use mem::size_of::<T>() and size_of_val for nested structs, and explain padding and alignment.
// How does memory layout affect performance?
use std::mem::{size_of, size_of_val};

#[repr(C)]
struct Packed {
    a: u8,  // 1 byte
    b: u32, // 4 bytes
    c: u16, // 2 bytes
}

#[repr(C)]
struct Optimized {
    b: u32, // 4 bytes
    c: u16, // 2 bytes
    a: u8,  // 1 byte
}

fn main() {
    // Simple types
    println!("size_of::<u8>() = {}", size_of::<u8>());
    println!("size_of::<u32>() = {}", size_of::<u32>());
    println!("size_of::<u64>() = {}", size_of::<u64>());

    // Nested struct
    let p = Packed { a: 1, b: 42, c: 7 };
    let o = Optimized { a: 1, b: 42, c: 7 };

    println!("size_of::<Packed>() = {}", size_of::<Packed>());
    println!("size_of::<Optimized>() = {}", size_of::<Optimized>());

    // size_of_val works at runtime
    println!("size_of_val(&p) = {}", size_of_val(&p));
    println!("size_of_val(&o) = {}", size_of_val(&o));

    // Arrays and slices
    let arr = [1u32, 2, 3, 4];
    println!("size_of_val(&arr) = {}", size_of_val(&arr));

    let slice: &[u32] = &arr;
    println!("size_of_val(slice) = {}", size_of_val(slice));
}
