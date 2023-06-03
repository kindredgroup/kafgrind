use std::collections::HashMap;

pub mod args;

pub fn generate_payload(size_in_bytes: u32) -> String {
    let mut payload = HashMap::new();
    let items = size_in_bytes * 1;
    for key in 0..items {
        payload.insert(key, key);
    }
    serde_json::to_string(&payload).unwrap()
}
