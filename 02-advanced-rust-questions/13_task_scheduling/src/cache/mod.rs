use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::{Arc, Weak};

type Link<K, V> = Option<Arc<RwLock<Node<K, V>>>>;

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Weak<RwLock<Node<K, V>>>>,
    next: Link<K, V>,
}

#[derive(Debug)]
pub struct LRUCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    capacity: usize,
    map: HashMap<K, Link<K, V>>,
    head: Link<K, V>, // most recently used
    tail: Link<K, V>, // Least recently used
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(node_rc) = self.map.get(key).and_then(|n| n.clone()) {
            let value = node_rc.read().unwrap().value.clone();
            self.move_to_head(&node_rc);
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(node_rc) = self.map.get(&key).and_then(|n| n.clone()) {
            node_rc.write().unwrap().value = value;
            self.move_to_head(&node_rc);
        } else {
            let new_node = Arc::new(RwLock::new(Node {
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

    fn move_to_head(&mut self, node: &Arc<RwLock<Node<K, V>>>) {
        self.remove_node(node);
        self.add_to_head(node);
    }

    fn add_to_head(&mut self, node: &Arc<RwLock<Node<K, V>>>) {
        node.write().unwrap().next = self.head.clone();
        node.write().unwrap().prev = None;

        if let Some(head) = &self.head {
            head.write().unwrap().prev = Some(Arc::downgrade(node));
        }
        self.head = Some(node.clone());

        if self.tail.is_none() {
            self.tail = Some(node.clone());
        }
    }

    fn remove_node(&mut self, node: &Arc<RwLock<Node<K, V>>>) {
        let prev = node.read().unwrap().prev.clone();
        let next = node.read().unwrap().next.clone();

        let prev_for_next = prev.clone();

        if let Some(prev) = prev.and_then(|w| w.upgrade()) {
            prev.write().unwrap().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next) = &next {
            next.write().unwrap().prev = prev_for_next;
        } else {
            self.tail = prev_for_next.and_then(|w| w.upgrade());
        }
        node.write().unwrap().prev = None;
        node.write().unwrap().next = None;
    }

    fn remove_tail(&mut self) {
        if let Some(tail_rc) = self.tail.take() {
            let key = tail_rc.read().unwrap().key.clone();
            self.remove_node(&tail_rc);
            self.map.remove(&key);
        }
    }
}
