use std::collections::HashMap;
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use rdkafka::Message;
use tokio::time::Instant;
use crate::kafka::kafka_client::{KafkaClient, KafkaClientType};
use crate::kafka::kafka_utils::{get_message_headers, get_raw_payload, get_time_stamp};
use crate::consumer::analyser::Analyser;

#[derive(Debug)]
pub struct TestMessage {
    pub publish_timestamp: u128,
    pub received_timestamp: u128,
    pub kafka_time_stamp: i64
}

pub struct Runner{
    kafka_client: KafkaClient,
    analyser: Analyser
}

impl Runner {
    pub fn new(analyser: Analyser, kafka_client: KafkaClient) -> Self {
        Runner {
            kafka_client,
            analyser,
        }
    }
    pub async fn consume_message(&mut self) -> TestMessage {
        let raw_message = self.kafka_client.consume_message().await;

        let headers = get_message_headers(&raw_message).expect("headers not found");
        let kafka_time_stamp = get_time_stamp(&raw_message);
        let publish_timestamp = headers
            .get("publish_timestamp").expect("missing message type").trim().parse().unwrap();;
        let received_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        TestMessage {
            publish_timestamp,
            received_timestamp,
            kafka_time_stamp
        }

    }

    pub async fn run(&mut self, number_of_records: u64) {
        self.kafka_client.subscribe();
        let mut interval = tokio::time::interval(Duration::from_millis(10000));
        let mut message_read_count = 0;
        let mut analysis_completed = false;
        let mut last_message_instant = std::time::Instant::now();
        while !analysis_completed {
            tokio::select! {
                message = self.consume_message() => {
                    self.analyser.update_data_set(message);
                    message_read_count = message_read_count+1;
                    last_message_instant = std::time::Instant::now();
                },
                _ = interval.tick() => {
                    self.analyser.update_view();
                    if message_read_count >= number_of_records {
                        analysis_completed = true;
                    }
                    if last_message_instant.elapsed() > Duration::from_secs(120) {
                        // In case no message received for last 120 secs. end the consumer thread.
                         analysis_completed = true;
                    }
                }
            }

        }
    }
}