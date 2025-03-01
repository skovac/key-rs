use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Type alias for the shared key-value store
pub type ThreadSafeHashmap = Arc<Mutex<HashMap<String, String>>>;

#[derive(Clone)]
pub struct Service {
    pub kv: KeyValueStore,
}

impl Service {
    pub fn new() -> Self {
        Service {
            kv: KeyValueStore::new(),
        }
    }
}

#[derive(Clone)]
pub struct KeyValueStore {
    hm: ThreadSafeHashmap,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            hm: Arc::new(Mutex::new(HashMap::<String, String>::new())),
        }
    }

    /// Create a new Service with an existing KeyValueStore
    pub fn _with_store(store: ThreadSafeHashmap) -> Self {
        KeyValueStore { hm: store }
    }

    /// Access the underlying KeyValueStore
    pub fn _store(&self) -> ThreadSafeHashmap {
        Arc::clone(&self.hm)
    }

    pub fn insert(self, key: String, value: String) {
        if let Ok(mut kv) = self.hm.lock() {
            kv.insert(key.clone(), value.clone());
        }
    }

    pub fn get(self, key: &str) -> Option<String> {
        if let Ok(kv) = self.hm.lock() {
            kv.get(key).cloned()
        } else {
            None
        }
    }

    pub fn remove(self, key: &str) -> Option<String> {
        if let Ok(mut hm) = self.hm.lock() {
            hm.remove(key)
        } else {
            None
        }
    }

    pub fn all(self) -> HashMap<String, String> {
        if let Ok(hm) = self.hm.lock() {
            hm.clone()
        } else {
            HashMap::new()
        }
    }

    pub fn keys(self) -> Keys {
        if let Ok(hm) = self.hm.lock() {
            hm.keys()
        }
    }
}
