use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Service {
    _kv: Arc<Mutex<HashMap<String, String>>>,
}

impl Default for Service {
    fn default() -> Self {
        Service {
            _kv: Arc::new(Mutex::new(HashMap::<String, String>::new())),
        }
    }
}
