// 72. How do you implement a custom allocator in Rust? Outline how to implement GlobalAlloc for a bump allocator in a toy no_std example.
// Why might you need a custom allocator?

// Why would you need a custom allocator?
// Rust normally relies on the system allocator (malloc/free) or on a library like jemalloc. But in some cases you can’t or don’t want to use that:
// no_std environments (embedded, kernels, OS dev) — no libc, so no malloc.
// Performance tuning — some workloads benefit from specialized allocators (e.g., bump allocators, slab allocators).
// Determinism — real-time systems may need predictable allocation behavior.
// Memory-constrained systems — e.g., microcontrollers with just a few KB of RAM.
// So you might implement a custom allocator to control memory directly.

use std::alloc::{GlobalAlloc, Layout};
use std::ptr::null_mut;

// Size of our heap
const HEAP_SIZE: usize = 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

// A simple bump allocator
struct BumpAllocator {
    next: usize,
    end: usize,
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut current = self.next;

        // Align up
        current = (current + layout.align() - 1) & !(layout.align() - 1);

        let new_next = current + layout.size();
        if new_next > self.end {
            return null_mut(); // Out of memory
        }

        let ptr = HEAP.as_ptr().add(current) as *mut u8;

        // Mutate self.next (since &self is immutable in signature)
        let self_mut = self as *const _ as *mut Self;
        (*self_mut).next = new_next;

        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // bump allocator cannot free memory
    }
}

// Install as global allocator
#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator {
    next: 0,
    end: HEAP_SIZE,
};

fn main() {
    println!("Custom bump allocator demo");

    // Use heap allocations
    let a = Box::new(10);
    let b = Box::new(20);
    println!("a = {}, b = {}", a, b);

    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);

    // Use a larger allocation
    let big = vec![42u8; 500];
    println!("Allocated big vec of len {}", big.len());

    // Try to exhaust heap
    let too_big = vec![0u8; 800]; // This should panic (OOM)
    println!("too_big = {:?}", too_big);
}
