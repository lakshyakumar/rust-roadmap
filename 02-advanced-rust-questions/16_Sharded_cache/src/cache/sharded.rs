use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::LRUCache;

// A sharded cache that partitions data across multiple LRUCache instances.
// Uses consistent hashing for shard selection.

#[derive(Debug)]
pub struct ShardedCache<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug + Send + Sync + 'static,
    V: Clone + std::fmt::Debug + Send + Sync + 'static,
{
    shards: Vec<Arc<RwLock<LRUCache<K, V>>>>,
    num_shards: usize,
}

impl<K, V> ShardedCache<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug + Send + Sync + 'static,
    V: Clone + std::fmt::Debug + Send + Sync + 'static,
{
    /// Create a new sharded cache with N shards
    pub fn new(num_shards: usize, shard_capacity: usize) -> Self {
        let mut shards = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            shards.push(Arc::new(RwLock::new(LRUCache::new(shard_capacity))));
        }

        Self { shards, num_shards }
    }

    /// Hash the key to find the correct shard index
    fn get_shard_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.num_shards
    }

    /// Get a value from the appropriate shard asynchronously
    pub async fn get(&self, key: &K) -> Option<V> {
        let index = self.get_shard_index(key);
        let mut shard = self.shards[index].write().await;
        shard.get(key)
    }

    /// Put a value into the appropriate shard asynchronously
    pub async fn put(&self, key: K, value: V) {
        let index = self.get_shard_index(&key);
        let mut shard = self.shards[index].write().await;
        shard.put(key, value);
    }

    /// Clear all shards asynchronously
    pub async fn clear(&self) {
        for shard in &self.shards {
            let mut s = shard.write().await;
            *s = LRUCache::new(s.capacity);
        }
    }
}
