// 17. Implement a wrapper type MyBox<T> that logs accesses to deref and deref_mut.
// How do you implement the Deref and DerefMut traits? What are the use cases for custom smart pointers?

use std::ops::{Deref, DerefMut};

struct MyBox<T> {
    value: T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        println!("MyBox::new created");
        MyBox { value }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("MyBox::deref called");
        &self.value
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("MyBox::deref_mut called");
        &mut self.value
    }
}

fn main() {
    let mut x = MyBox::new(42);

    // Immutable deref
    println!("Value = {}", *x);

    // Mutable deref
    *x = 100;
    println!("Updated Value = {}", *x);

    // Works with functions expecting &T due to deref coercion
    print_length(&MyBox::new(String::from("Hello")));
}

fn print_length(s: &str) {
    println!("Length = {}", s.len());
}
