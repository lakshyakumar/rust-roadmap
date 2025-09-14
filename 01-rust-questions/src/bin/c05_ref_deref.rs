// ref, deref and deref mut in rust

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // Ref meanning
    let a = 10;
    let ref b = a;
    let c = &a;

    println!("a==*b is {}, b==c is {}", a == *b, b == c);

    // ref use case
    let a = Some(String::from("world!"));

    match a {
        Some(x) => println!("Hello {}", x),
        None => println!("Nothing"),
    }

    // println!("{:?}", a); // Error due to move

    let b = Some(String::from("world!"));

    match b {
        Some(ref x) => println!("Hello {}", x),
        None => println!("Nothing"),
    }
    println!("{:?}", b);

    // Deref
    let mb = MyBox::new(5);
    let b = *mb;

    println!("b is {}", b);
}
