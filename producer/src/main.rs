use std::time::Duration;
use chrono::Utc;
use producer::run_producer_lib;


fn main() {
    /// rate and for the time in minutes
    let rate_map:Vec<(u64, u64)> = Vec::from([
        (200,2),
        (500,2),
        (1000,2),
        (1500,2),
        (2000,2),
        (3000,2),
        (4000,2),
        (5000,2),
        (6000,2),
        (7000,2),
        (8000,2),
        (9000,2)
    ]);
    let sleep_between_run = 20; // 30 secs
    run_producer_lib(rate_map, sleep_between_run);
}
