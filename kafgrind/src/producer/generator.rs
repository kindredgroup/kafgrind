use crate::producer::models::Executor;
use crate::kafka::kafka_client::KafkaClient;
use std::thread;
use std::time::{Duration, Instant};
use time::OffsetDateTime;
/// Generate the load at controlled rate
pub struct Generator {
    executor: Box<dyn Executor>,
    rate: u64,
    number_of_records: u64,
}

impl Generator {
    pub fn new(executor: Box<dyn Executor>, rate: u64, number_of_records: u64) -> Self {
        Self {
            executor,
            rate,
            number_of_records,
        }
    }
    /// Calls self.executor.execute at defined self.rate
    pub fn generate(&self) {
        let adjusted_rate = self.rate as f64 /1.0;
        let started_at = OffsetDateTime::now_utc().unix_timestamp_nanos();
        let adjust_every = 5;
        let mut delay = Duration::from_nanos((1_000_000_000 as f64 / adjusted_rate) as u64);
        let mut done = 0;
        let mut skip = 0;
        let min_sleep = Duration::from_micros(100);
        while done < self.number_of_records {
            done += 1;
            if skip == 0 {
                thread::sleep(delay);
            } else {
                skip -= 1
            }

            self.executor.execute();
            if done % adjust_every == 0 {
                let now = OffsetDateTime::now_utc().unix_timestamp_nanos();
                let elapsed = Duration::from_nanos((now - started_at) as u64).as_secs_f64();
                let effective_rate = done as f64 / elapsed;
                let scale = adjusted_rate / effective_rate;
                let sleep = delay.as_secs_f64() / scale;
                if Duration::from_secs_f64(sleep) < min_sleep {
                    skip = (min_sleep.as_secs_f64() / sleep).ceil() as u64 * adjust_every;
                } else {
                    delay = Duration::from_secs_f64(sleep);
                    skip = 0;
                }
            }
        }
    }
}
