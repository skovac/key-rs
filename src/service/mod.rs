use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Service {
    kv: Arc<Mutex<HashMap<String, String>>>,
}

impl Default for Service {
    fn default() -> Self {
        Service {
            kv: Arc::new(Mutex::new(HashMap::<String, String>::new())),
        }
    }
}

impl Clone for Service {
    fn clone(&self) -> Self {
        Service {
            kv: self.kv.clone(),
        }
    }
}

impl Service {
    pub fn get_all(self) -> Arc<Mutex<HashMap<String, String>>> {
        Arc::clone(&self.kv)
    }
}
