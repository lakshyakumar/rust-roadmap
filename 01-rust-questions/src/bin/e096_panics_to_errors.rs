// 96. How do you convert panicking code to return Result instead?
// Use std::panic::catch_unwind where appropriate and document invariants. Why is error handling preferred over panics?

use std::panic;

fn get_element(vec: &[i32], index: usize) -> Result<i32, String> {
    if index >= vec.len() {
        Err(format!("Index {} out of bounds (len={})", index, vec.len()))
    } else {
        Ok(vec[index])
    }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    let result = panic::catch_unwind(|| {
        // potentially panicking code
        a.checked_div(b).unwrap() // would panic on divide by zero
    });

    match result {
        Ok(val) => Ok(val),
        Err(_) => Err("Division panicked (possibly divide by zero)".into()),
    }
}

fn main() {
    let v = vec![1, 2, 3];
    match get_element(&v, 10) {
        Ok(val) => println!("Value: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match safe_divide(10, 0) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Caught error: {}", e),
    }
}
