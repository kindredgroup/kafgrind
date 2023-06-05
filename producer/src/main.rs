use chrono::Utc;
use producer::{run_base_producer_task};
use std::time::Duration;

fn main() {
    run_base_producer_task(20000, 1000, "localhost:9092".to_owned(), "test.topic".to_owned(), "{\"k\":\"v\"}".to_owned(), None, None);
}
