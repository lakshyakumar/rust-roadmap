// 60. How do you implement a lock-free stack (Treiber stack) using AtomicPtr?
// Ensure safe Drop on pop operations. What are the pitfalls of manual memory management?

// push creates a new node and links it in atomically.
// pop removes a node and safely drops its value.
// We must carefully handle memory, because we’re dealing with raw pointers.

use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

pub struct TreiberStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> TreiberStack<T> {
    pub fn new() -> Self {
        TreiberStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: ptr::null_mut(),
        });
        let new_node_ptr = Box::into_raw(new_node);

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node_ptr).next = head;
            }

            if self
                .head
                .compare_exchange(head, new_node_ptr, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                break;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };

            if self
                .head
                .compare_exchange(head, next, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                unsafe {
                    let boxed = Box::from_raw(head);
                    return Some(boxed.value);
                }
            }
        }
    }
}

impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

fn main() {
    let stack = Arc::new(TreiberStack::new());

    let s1 = stack.clone();
    let t1 = thread::spawn(move || {
        for i in 0..5 {
            s1.push(i);
        }
    });

    let s2 = stack.clone();
    let t2 = thread::spawn(move || {
        for i in 5..10 {
            s2.push(i);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    // Now safely pop
    while let Some(v) = stack.pop() {
        println!("Popped: {}", v);
    }
}
