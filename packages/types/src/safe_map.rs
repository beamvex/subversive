use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A thread-safe map implementation that maintains a read-optimized copy.
/// Writes are protected by a RwLock, while reads use an Arc to access
/// an immutable snapshot of the data.
pub struct SafeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    // The internal map protected by RwLock for write operations
    internal_map: RwLock<HashMap<K, V>>,
    // A read-only copy that can be accessed without locking
    readonly_copy: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> SafeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a new empty SafeMap
    pub fn new() -> Self {
        let map = HashMap::new();
        SafeMap {
            internal_map: RwLock::new(map.clone()),
            readonly_copy: Arc::new(RwLock::new(map)),
        }
    }

    /// Creates a new SafeMap with the given initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let map = HashMap::with_capacity(capacity);
        SafeMap {
            internal_map: RwLock::new(map.clone()),
            readonly_copy: Arc::new(RwLock::new(map)),
        }
    }

    /// Gets a reference to the read-only copy of the map
    pub async fn readonly(&self) -> tokio::sync::RwLockReadGuard<'_, HashMap<K, V>> {
        self.readonly_copy.read().await
    }

    /// Updates the read-only copy after a mutation
    async fn update_readonly_copy(&self) {
        let internal_guard = self.internal_map.read().await;
        let mut readonly_guard = self.readonly_copy.write().await;
        *readonly_guard = internal_guard.clone();
    }

    /// Inserts a key-value pair into the map
    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let mut guard = self.internal_map.write().await;
        let result = guard.insert(key, value);
        drop(guard);
        self.update_readonly_copy().await;
        result
    }

    /// Removes a key from the map
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut guard = self.internal_map.write().await;
        let result = guard.remove(key);
        drop(guard);
        self.update_readonly_copy().await;
        result
    }

    /// Clears the map, removing all key-value pairs
    pub async fn clear(&self) {
        let mut guard = self.internal_map.write().await;
        guard.clear();
        drop(guard);
        self.update_readonly_copy().await;
    }

    /// Updates multiple entries atomically
    pub async fn update_many<I>(&self, entries: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut guard = self.internal_map.write().await;
        for (key, value) in entries {
            guard.insert(key, value);
        }
        drop(guard);
        self.update_readonly_copy().await;
    }

    /// Gets a read lock on the internal map if more complex operations are needed
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, HashMap<K, V>> {
        self.internal_map.read().await
    }

    /// Gets a write lock on the internal map if more complex operations are needed
    pub async fn write(&self) -> tokio::sync::RwLockWriteGuard<'_, HashMap<K, V>> {
        let guard = self.internal_map.write().await;
        self.update_readonly_copy().await;
        guard
    }
}

impl<K, V> Default for SafeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for SafeMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            internal_map: RwLock::new(self.internal_map.blocking_read().clone()),
            readonly_copy: Arc::clone(&self.readonly_copy),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_operations() {
        let map = SafeMap::new();

        // Test insert
        map.insert("key1".to_string(), 1).await;
        assert_eq!(map.readonly().await.get("key1"), Some(&1));

        // Test remove
        assert_eq!(map.remove(&"key1".to_string()).await, Some(1));
        assert_eq!(map.readonly().await.get("key1"), None);

        // Test update_many
        let entries = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ];
        map.update_many(entries).await;
        assert_eq!(map.readonly().await.len(), 3);
        assert_eq!(map.readonly().await.get("b"), Some(&2));
    }

    #[tokio::test]
    async fn test_clear() {
        let map = SafeMap::new();
        map.insert("key1".to_string(), 1).await;
        map.insert("key2".to_string(), 2).await;
        assert_eq!(map.readonly().await.len(), 2);

        map.clear().await;
        assert_eq!(map.readonly().await.len(), 0);
    }
}
