// 73. How do you build a crate with #![no_std], use only the core library, and expose a function usable in embedded environments?
// How do you unit test with std via cfg? What are the constraints of no_std?

// Rust crates use the standard library (std), which depends on an OS and libc (file I/O, networking, threads, heap allocator, etc.).
// But on embedded systems, kernels, or bare-metal targets, there is no OS → std is unavailable.
// Instead, you use:
// core: always available, contains primitives (slices, Option, Result, Iterator, math ops, etc.).
// alloc: available if you have a memory allocator (gives you Box, Vec, String).
// std: only if an OS/libc is present.
// So, in embedded, you write #![no_std] to disable std and only rely on core.

#![no_std] // forbid linking std

// Example function usable in embedded
pub fn add_two(x: u32) -> u32 {
    x + 2
}
fn main() {}

// #![no_std] disables std globally.
// Inside #[cfg(test)], we explicitly extern crate std; so unit tests can use std features like assertions.

// Only compile tests when using std (on host machine)
#[cfg(test)]
mod tests {
    // Pull in std just for testing
    extern crate std;

    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(3), 5);
    }
}
