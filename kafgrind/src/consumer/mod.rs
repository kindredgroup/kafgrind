

use std::collections::HashMap;
use crate::kafka::kafka_client::{KafkaClient, KafkaClientType};
use crate::consumer::analyser::Analyser;
use crate::consumer::models::View;
use crate::consumer::runner::Runner;

pub mod runner;
pub mod analyser;
pub mod models;
pub mod view;

pub async fn run_consumer(number_of_records: u64, brokers: String, topic: String, partition_length: u8, kafka_user: Option<String>, kafka_password: Option<String>) {
    let view = View{};
    let analyser = Analyser::new(view);
    let override_config: HashMap<&str, &str> = HashMap::from([
        // ("fetch.max.wait.ms", "500"),
        // ("fetch.min.bytes", "100"),
        ("fetch.max.bytes", "52428800"),
        ("max.partition.fetch.bytes", "1048576"),
        ("group.id", "kafgrind_analyser"),
        ("auto.offset.reset", "latest"),
        ("enable.auto.commit", "true"),
        ("auto.commit.interval.ms", "10000"),
        ]);
    let kafka_client = KafkaClient::new(KafkaClientType::Consumer, Some(override_config), brokers, topic, Some(partition_length), kafka_user, kafka_password);
    let mut runner = Runner::new(analyser, kafka_client);
    runner.run(number_of_records).await;
}
