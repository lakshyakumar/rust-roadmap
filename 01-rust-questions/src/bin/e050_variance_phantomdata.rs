// 50. How do you explore variance in Rust using PhantomData?
// Show examples of invariance and covariance, and explain why &T is covariant while Cell<T> is invariant.
// Why does variance matter?

// Variance describes how subtyping of type parameters affects subtyping of the containing type.
// Covariant: Container<'a> can be substituted with a shorter lifetime safely.
// Contravariant: Container<'a> can be substituted with a longer lifetime safely.
// Invariant: No substitution is allowed; type must match exactly.
// In Rust, variance mostly matters for lifetimes, and PhantomData is used to tell the compiler about variance when the type parameter isn’t actually used.

// PhantomData<T> lets Rust know: “this struct logically owns/borrows a T, even if it doesn’t store it.”

use std::cell::Cell;
use std::marker::PhantomData;

// A type that is covariant over its lifetime parameter `'a`
// because &T is covariant.
struct Covariant<'a, T> {
    _marker: PhantomData<&'a T>,
}

// A type that is invariant over its lifetime parameter `'a`
// because Cell<T> is invariant.
struct Invariant<'a, T> {
    _marker: PhantomData<Cell<&'a T>>,
}

// Function showing covariance: we can shorten the lifetime.
fn covariance<'short, 'long: 'short>(c: Covariant<'long, u32>) -> Covariant<'short, u32> {
    c // ✅ Allowed: Covariant is covariant in 'a
}

// // Function showing invariance: we cannot shorten the lifetime.
// fn invariance<'short, 'long: 'short>(i: Invariant<'long, u32>) -> Invariant<'short, u32> {
//     i // ❌ ERROR: Invariant is invariant in 'a
// }

fn main() {
    let long = 42;
    let short = 7;

    // Demonstrate covariance
    {
        let cov_long: Covariant<'_, u32> = Covariant {
            _marker: PhantomData,
        };
        let _cov_short: Covariant<'_, u32> = covariance(cov_long);
        println!("Covariance works: &'long T can be used as &'short T");
    }

    // Demonstrate invariance (uncomment to see the compiler error)
    {
        let inv_long: Invariant<'_, u32> = Invariant {
            _marker: PhantomData,
        };
        // let _inv_short: Invariant<'_, u32> = invariance(inv_long);
        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // This will fail to compile:
        // "error: lifetime may not live long enough"
        println!("Invariance prevents lifetime coercion (cannot compile).");
    }

    println!("Done!");
}
