// 70. How do you build an LRU cache using a HashMap and a doubly linked list (Rc<RefCell<Node>> or raw pointers with unsafe for performance)?
// Why is LRU caching important?
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Weak<RefCell<Node<K, V>>>>,
    next: Link<K, V>,
}

#[derive(Debug)]
struct LRUCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    capacity: usize,
    map: HashMap<K, Link<K, V>>,
    head: Link<K, V>, // Most recently used
    tail: Link<K, V>, // Least recently used
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> LRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: &K) -> Option<V> {
        if let Some(node_rc) = self.map.get(key).and_then(|n| n.clone()) {
            let value = node_rc.borrow().value.clone();
            self.move_to_head(&node_rc);
            Some(value)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if let Some(node_rc) = self.map.get(&key).and_then(|n| n.clone()) {
            node_rc.borrow_mut().value = value;
            self.move_to_head(&node_rc);
        } else {
            let new_node = Rc::new(RefCell::new(Node {
                key: key.clone(),
                value,
                prev: None,
                next: None,
            }));

            if self.map.len() == self.capacity {
                self.remove_tail();
            }

            self.add_to_head(&new_node);
            self.map.insert(key, Some(new_node));
        }
    }

    fn move_to_head(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        self.remove_node(node);
        self.add_to_head(node);
    }

    fn add_to_head(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        node.borrow_mut().next = self.head.clone();
        node.borrow_mut().prev = None;

        if let Some(head) = &self.head {
            head.borrow_mut().prev = Some(Rc::downgrade(node));
        }
        self.head = Some(node.clone());

        if self.tail.is_none() {
            self.tail = Some(node.clone());
        }
    }

    fn remove_node(&mut self, node: &Rc<RefCell<Node<K, V>>>) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        let prev_for_next = prev.clone();

        if let Some(prev) = prev.and_then(|w| w.upgrade()) {
            prev.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next) = &next {
            next.borrow_mut().prev = prev_for_next;
        } else {
            self.tail = prev_for_next.and_then(|w| w.upgrade());
        }

        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
    }

    fn remove_tail(&mut self) {
        if let Some(tail_rc) = self.tail.take() {
            let key = tail_rc.borrow().key.clone();
            self.remove_node(&tail_rc);
            self.map.remove(&key);
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(3);

    cache.put(1, "a");
    cache.put(2, "b");
    cache.put(3, "c");

    println!("{:?}", cache.get(&2)); // Some("b") -> moves key 2 to head
    cache.put(4, "d"); // Evicts key 1 (least recently used)

    println!("{:?}", cache.get(&1)); // None
    println!("{:?}", cache.get(&3)); // Some("c")
    println!("{:?}", cache.get(&4)); // Some("d")
}
