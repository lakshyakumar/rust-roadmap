// 12. Explain associated types in traits. Define a trait Storage with associated types Key and Value, and implement it for a HashMapStorage.
// How do associated types improve trait flexibility?

use std::collections::HashMap;

trait Storage {
    type Key;
    type Value;

    fn insert(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

struct HashMapStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Storage for HashMapStorage<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    type Key = K;
    type Value = V;

    fn insert(&mut self, key: Self::Key, value: Self::Value) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.data.get(&key)
    }
}

fn main() {
    let mut store = HashMapStorage {
        data: HashMap::new(),
    };

    store.insert("apple", 3);
    store.insert("banana", 5);

    if let Some(val) = store.get(&"apple") {
        println!("apple => {}", val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut store = HashMapStorage {
            data: HashMap::new(),
        };
        store.insert("apple", 10);
        store.insert("banana", 20);
        assert_eq!(store.get(&"apple"), Some(&10));
        assert_eq!(store.get(&"banana"), Some(&20));
        assert_eq!(store.get(&"pear"), None);
    }

    #[test]
    fn test_overwrite_value() {
        let mut store = HashMapStorage {
            data: HashMap::new(),
        };
        store.insert("apple", 1);
        store.insert("apple", 2);
        assert_eq!(store.get(&"apple"), Some(&2));
    }
}
