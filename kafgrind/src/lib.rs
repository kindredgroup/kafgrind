use std::collections::HashMap;



pub mod args;
pub mod kafka;
pub mod producer;
pub mod consumer;

pub fn generate_payload(size_in_bytes: usize) -> String {
    let mut payload = String::with_capacity(size_in_bytes);
    let mut index = 0;
    while index < payload.capacity() {
        payload.push('a');
        index = index + 1;
    }
    payload
}
