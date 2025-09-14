// 16. How do you build a tree structure using Rc<RefCell<T>> for nodes with children?
// Implement an add_child method that updates the parent via Weak. Discuss the trade-offs of using reference counting and interior mutability.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }
    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        parent.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&root, child1.clone());
    Node::add_child(&root, child2.clone());

    println!("Root: {:?}", root.value);
    println!(
        "Children: {:?}",
        root.children
            .borrow()
            .iter()
            .map(|c| c.value)
            .collect::<Vec<_>>()
    );

    // Upgrade Weak -> Rc to access parent
    let weak_parent = child1.parent.borrow().clone(); // borrow ends here
    if let Some(parent) = weak_parent.upgrade() {
        println!("Child1's parent = {}", parent.value);
    }
}
