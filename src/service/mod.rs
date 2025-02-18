use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Service {
    kv: Arc<Mutex<HashMap<String, String>>>,
}

impl Service {
    pub fn new() -> Self {
        Service {
            kv: Arc::new(Mutex::new(HashMap::<String, String>::new())),
        }
    }

    pub fn insert(self, key: String, value: String) {
        // let kv_cloned = Arc::clone(&self.kv);
        //        let mut kv_unlocked = self.clone().kv.lock().unwrap();
        //        kv_unlocked.insert(key, value);
        //        self.clone().kv.lock().unwrap().insert(key, value);
        if let Ok(mut kv) = self.kv.lock() {
            //            kv.insert(key, value);
            for _i in 0..10 {
                kv.insert(key.clone(), value.clone());
            }
        }
    }
}
