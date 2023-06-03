use std::collections::HashMap;
use rdkafka::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::{Header, OwnedHeaders, BorrowedMessage};
use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
use rdkafka::error::KafkaError;
use rdkafka::types::RDKafkaErrorCode;
use crate::kafka_utils::build_kafka_headers;
const MAX_RETRY_COUNT: u32 = 10;
pub struct KafkaClient {
    consumer: Option<StreamConsumer>,
    producer: Option<ThreadedProducer<DefaultProducerContext>>,
}
pub enum KafkaClientType {
    Producer,
    Consumer
}

impl KafkaClient {
    pub fn new(client_type: KafkaClientType, config_overrides: Option<HashMap<&str, &str>>) -> Self {
        let mut client_config = ClientConfig::new();
        let mut base_config = HashMap::from([
            ("bootstrap.servers","localhost:9092"),
        ]);
        // let mut base_config = HashMap::from([
        //     ("bootstrap.servers","kb001.ksp-int.syd1.kc.thinkbig.local,kb002.ksp-int.syd1.kc.thinkbig.local,kb003.ksp-int.syd1.kc.thinkbig.local,kb004.ksp-int.syd1.kc.thinkbig.local,kb005.ksp-int.syd1.kc.thinkbig.local"),
        //     ("security.protocol", "SASL_PLAINTEXT"),
        //     ("sasl.mechanisms", "SCRAM-SHA-512"),
        //     ("sasl.username", "talos-certifier"),
        //     ("sasl.password", "mhgau4UDByKJxcECCYmhwpD8p5IsoQoN")
        // ]);
        if let Some(overrides) = config_overrides {
            base_config.extend(overrides);
        }
        for (k,v) in base_config.into_iter() {
            client_config.set(k,v);
        }
        match client_type {
            KafkaClientType::Producer => {
                let producer: ThreadedProducer<DefaultProducerContext> = client_config.create().expect("Failed to create producer");
                Self {
                    producer: Some(producer),
                    consumer: None
                }
            },
            KafkaClientType::Consumer => {
                let consumer: StreamConsumer = client_config.create().expect("Failed to create consumer");
                Self {
                    producer: None,
                    consumer: Some(consumer)
                }
            }
        }
    }
    pub fn publish_message(&self, key: &str, value: String, headers: Option<HashMap<String, String>>, topic: &str) {
        let record = BaseRecord::to(topic).payload(&value).key(key);

        let mut record = match headers.clone() {
            Some(x) => record.headers(build_kafka_headers(x)),
            None => record,
        };

        loop {
            match self.producer.as_ref().unwrap().send(record) {
                Ok(()) => {
                    break;
                }
                Err((KafkaError::MessageProduction(RDKafkaErrorCode::QueueFull), rec)) => {
                    record = rec;
                    eprintln!("QueueFull");
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                Err((e, _)) => {
                    eprintln!("Failed to publish on kafka {:?}", e);
                    break;
                }
            }
        }
    }
    pub fn subscribe(&self, topic: &str) {
        &self.consumer.as_ref()
            .unwrap().subscribe(&[topic]).expect("unable to subscribe to topic");
    }

    pub fn unsubscribe(&self) {
        self.consumer.as_ref().unwrap().unsubscribe();
    }
    pub async fn consume_message(&mut self) -> BorrowedMessage {
        self.consumer.as_mut().unwrap().recv().await.expect("message recv error")
    }
}



