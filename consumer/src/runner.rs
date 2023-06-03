use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use rdkafka::Message;
use kafka::kafka_client::{KafkaClient, KafkaClientType};
use models::talos_models::{CandidateMessage, DecisionMessage, DecisionType, MessageType};
use kafka::kafka_utils::{get_message_headers, get_raw_payload, get_time_stamp};
use models::talos_models::MessageType::{Candidate, Decision};
use tokio::sync::mpsc;
use crate::analyser::Analyser;

#[derive(Debug)]
pub struct TestMessage {
    pub id: String,
    pub message_type: MessageType,
    pub time: i64,
    pub decision: Option<DecisionType>,
    pub test_id: Option<String>
}

pub struct Runner{
    kafka_client: KafkaClient,
    topic: String,
    analyser: Analyser
}

impl Runner {
    pub fn new(analyser: Analyser) -> Self {
        let override_config: HashMap<&str, &str> = HashMap::from([
            // ("fetch.max.wait.ms", "500"),
            // ("fetch.min.bytes", "100"),
            ("fetch.max.bytes", "52428800"),
            ("max.partition.fetch.bytes", "1048576"),
            ("group.id", "analyser"),
            ("auto.offset.reset", "latest"),
            ("auto.commit.interval.ms", "1000"),
            ("enable.auto.commit", "true"),
            ]);
        let kafka_client = KafkaClient::new(KafkaClientType::Consumer, Some(override_config));
        Runner {
            kafka_client,
            analyser,
            topic: "int.ksp.certification".to_string()
        }
    }
    pub async fn consume_message(&mut self) -> Option<TestMessage> {
        let raw_message = self.kafka_client.consume_message().await;

        let headers = get_message_headers(&raw_message).expect("headers not found");
        let msg = get_raw_payload(&raw_message);
        let time_stamp = get_time_stamp(&raw_message);
        let msg_type = MessageType::from_str(headers
            .get("messageType").expect("missing message type"));

        return match msg_type {
            Ok(Candidate) => {
                let msg_value: CandidateMessage = serde_json::from_slice(msg).expect("unexpected candidate message");
                let test_id = headers.get("testId").expect("missing testId Header");
                let msg = TestMessage {
                    decision: None,
                    message_type: Candidate,
                    id: msg_value.xid,
                    time: time_stamp,
                    test_id: Some(test_id.to_owned())
                };
                Some(msg)
            },
            Ok(Decision) => {
                let msg_value: DecisionMessage = serde_json::from_slice(msg).expect("unexpected decision message");
                let msg = TestMessage {
                    decision: Some(msg_value.decision),
                    message_type: Decision,
                    id: msg_value.xid,
                    time: time_stamp,
                    test_id: None
                };
                Some(msg)
            },
            Ok(_) => {
                None
            }
            Err(_) => {
                None
            }
        }
    }

    pub async fn run(&mut self) {
        self.kafka_client.subscribe(self.topic.as_str());
        let mut interval = tokio::time::interval(Duration::from_millis(30000));
        loop {
            tokio::select! {
                message = self.consume_message() => {
                    if let Some(mess) = message {
                        self.analyser.update_data_set(mess);
                    }
                },
                _ = interval.tick() => {
                    self.analyser.update_view();
                }
            }

        }
    }
}