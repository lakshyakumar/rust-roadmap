// Rc, RefCell, and Rc<RefCell<T>>

// Rc<T> → Reference Counting Smart Pointer
// Multiple immutable owners of the same data.
// Not thread-safe (for multi-threading, you’d use Arc).

// RefCell<T> → Interior Mutability
// Lets you mutate data even when it’s immutable.
// Enforced at runtime (not compile-time).
// Panics if you break borrow rules (e.g., two mutable borrows at once).

// Rc<T> → when you need multiple read-only owners.
// RefCell<T> → when you need mutable data inside an immutable wrapper.
// Rc<RefCell<T>> → when you need multiple owners who can also mutate the same data.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // usage for RC
    let data = Rc::new(String::from("Shared data"));
    {
        let a = Rc::clone(&data);
        let b = Rc::clone(&data);

        println!("a = {}, b = {}", a, b);
        println!("Reference count = {}", Rc::strong_count(&data));
    }
    println!("Reference count = {}", Rc::strong_count(&data));

    // usage for RefCell
    let data = RefCell::new(5);
    println!("value = {}", data.borrow()); // as it is a RefCell type

    *data.borrow_mut() = 10;
    println!("value = {}", data.borrow());

    // usage for both RC and RefCell
    let shared = Rc::new(RefCell::new(0));

    {
        let a = Rc::clone(&shared);
        let b = Rc::clone(&shared);

        *a.borrow_mut() += 5;

        println!("b sees value = {}", b.borrow());
        *b.borrow_mut() += 10;
    }
    println!("Final value for shared data : {}", shared.borrow());
}
