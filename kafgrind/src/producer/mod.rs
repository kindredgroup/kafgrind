pub mod base_executor;
pub mod generator;
pub mod models;

use std::collections::HashMap;
use crate::kafka::kafka_client::{KafkaClient, KafkaClientType};
use crate::producer::generator::Generator;
use crate::producer::base_executor::BaseExecutor;

pub fn run_base_producer_task(
    rate: u64,
    number_of_records: u64,
    brokers: String,
    topic: String,
    message: String,
    kafka_user: Option<String>,
    kafka_password: Option<String>
) {
    let num_of_threads: u64 = match rate {
        1..=1000 => 1,
        0 => panic!(" rate cannot be 0"),
        _ => 10
    };

    let mut threads = Vec::with_capacity(num_of_threads as usize);
    // println!(" starting test for rate: {rate} msgs/sec for time: {time} mins on num_of_threads:{num_of_threads}");
    for thread_id in 0..num_of_threads {
        let rate_per_thread = rate / num_of_threads;
        let base_count_per_thread = number_of_records / num_of_threads;
        let extra_records = number_of_records % num_of_threads;
        let message_clone = message.clone();
        let topic_clone = topic.clone();
        let brokers_clone = brokers.clone();
        let kafka_user_clone = kafka_user.clone();
        let kafka_password_clone = kafka_password.clone();
        let record_count_in_current_thread = if thread_id == num_of_threads -1 {base_count_per_thread + extra_records} else {base_count_per_thread};
        threads.push(std::thread::spawn(move || {
            let override_config: HashMap<&str, &str> = HashMap::from([
                ("batch.size", "100000"),
                ("linger.ms", "5"),
                ("message.timeout.ms", "50000"),
                ("socket.keepalive.enable", "true"),
                ("message.send.max.retries", "100000"),
                ("topic.metadata.refresh.interval.ms", "4")
                // ("compression.type","lz4")
            ]);
            let kafka_client = KafkaClient::new(
                KafkaClientType::Producer,
                Some(override_config),
                brokers_clone,
                topic_clone,
                None,
                kafka_user_clone,
                kafka_password_clone
            );

            let base_executor = BaseExecutor::new(kafka_client, message_clone);
            let generator = Generator::new(Box::new(base_executor), rate_per_thread, record_count_in_current_thread);
            generator.generate();
        }));
    }
    // Wait for all threads to finish.
    for handle in threads {
        handle.join().unwrap();
    }
}
