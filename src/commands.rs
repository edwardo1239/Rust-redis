use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn set(storage: &Arc<Mutex<HashMap<String, String>>>,key:String, value:String){
    let mut map = storage.lock().unwrap();
    map.insert(key, value);
}

pub fn get(storage: &Arc<Mutex<HashMap<String, String>>>, key:&str) -> Option<String> {
    let map = storage.lock().unwrap();
    map.get(key).cloned()
}