// 52. How do you use Any and downcast in Rust? Create a heterogeneous Vec<Box<dyn Any>>,
// push different types, and retrieve them by type using downcast_ref.
// What are the use cases for type erasure?
use std::any::Any;

fn main() {
    // A heterogeneous collection: Vec of boxed dyn Any
    let mut items: Vec<Box<dyn Any>> = Vec::new();

    // Push values of different types
    items.push(Box::new(42i32));
    items.push(Box::new("hello".to_string()));
    items.push(Box::new(3.14f64));

    // Iterate and try to retrieve each by type
    for item in &items {
        if let Some(int_val) = item.downcast_ref::<i32>() {
            println!("Found i32: {}", int_val);
        } else if let Some(str_val) = item.downcast_ref::<String>() {
            println!("Found String: {}", str_val);
        } else if let Some(float_val) = item.downcast_ref::<f64>() {
            println!("Found f64: {}", float_val);
        } else {
            println!("Unknown type");
        }
    }
}
