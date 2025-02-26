use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Type alias for the shared key-value store
pub type KeyValueStore = Arc<Mutex<HashMap<String, String>>>;

#[derive(Clone)]
pub struct Service {
    kv: KeyValueStore,
}

impl Service {
    pub fn new() -> Self {
        Service {
            kv: Arc::new(Mutex::new(HashMap::<String, String>::new())),
        }
    }

    /// Create a new Service with an existing KeyValueStore
    pub fn _with_store(store: KeyValueStore) -> Self {
        Service { kv: store }
    }

    /// Access the underlying KeyValueStore
    pub fn _store(&self) -> KeyValueStore {
        Arc::clone(&self.kv)
    }

    pub fn insert(&self, key: String, value: String) {
        if let Ok(mut kv) = self.kv.lock() {
            for _i in 0..10 {
                kv.insert(key.clone(), value.clone());
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        if let Ok(kv) = self.kv.lock() {
            kv.get(key).cloned()
        } else {
            None
        }
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        if let Ok(mut kv) = self.kv.lock() {
            kv.remove(key)
        } else {
            None
        }
    }

    pub fn list(&self) -> HashMap<String, String> {
        if let Ok(kv) = self.kv.lock() {
            kv.clone()
        } else {
            HashMap::new()
        }
    }
}
