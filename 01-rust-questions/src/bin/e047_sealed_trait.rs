// 47. How do you implement the sealed trait pattern in Rust?
// Create a trait in a module with a private Sealed type to prevent external implementations, and explain the rationale.
// What problems does sealing solve?

// Rust doesn’t have a sealed keyword like some languages (C#), but we can emulate it using a private trait.
// Idea: make a trait depend on a private type only accessible inside the module.
// External code can see the trait, but cannot implement it because they cannot access the private type.
// ✅ Useful when you want to extend traits safely without breaking API guarantees.

mod shapes {
    // 1️⃣ Private "sealed" trait
    mod sealed {
        pub trait Sealed {}
    }

    // 2️⃣ Public trait depends on the private sealed trait
    pub trait Shape: sealed::Sealed {
        fn area(&self) -> f64;
    }

    // 3️⃣ Internal types
    pub struct Circle {
        pub radius: f64,
    }

    pub struct Square {
        pub side: f64,
    }

    // 4️⃣ Implement the private Sealed trait internally
    impl sealed::Sealed for Circle {}
    impl sealed::Sealed for Square {}

    // 5️⃣ Implement public trait
    impl Shape for Circle {
        fn area(&self) -> f64 {
            3.1415 * self.radius * self.radius
        }
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }
}

// 6️⃣ Usage
fn print_area<S: shapes::Shape>(shape: &S) {
    println!("Area = {}", shape.area());
}

fn main() {
    let c = shapes::Circle { radius: 2.0 };
    let s = shapes::Square { side: 3.0 };

    print_area(&c);
    print_area(&s);

    // ❌ External code cannot implement Shape because Sealed is private:
    // struct Triangle;
    // impl shapes::Shape for Triangle { ... } // ❌ won't compile
}
