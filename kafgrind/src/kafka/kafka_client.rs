use crate::kafka::kafka_utils::build_kafka_headers;
use rdkafka::consumer::{Consumer, RebalanceProtocol, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::{BorrowedMessage, Header, OwnedHeaders};
use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
use rdkafka::types::RDKafkaErrorCode;
use rdkafka::{ClientConfig, Offset, TopicPartitionList};
use std::collections::HashMap;
use std::time::Duration;


pub struct KafkaClient {
    consumer: Option<StreamConsumer>,
    producer: Option<ThreadedProducer<DefaultProducerContext>>,
    topic: String,
    partition_length: Option<u8>
}
pub enum KafkaClientType {
    Producer,
    Consumer,
}

impl KafkaClient {
    pub fn new(
        client_type: KafkaClientType,
        config_overrides: Option<HashMap<&str, &str>>,
        servers: String,
        topic: String,
        partition_length: Option<u8>,
        kafka_user: Option<String>,
        kafka_password: Option<String>
    ) -> Self {
        let mut client_config = ClientConfig::new();
        let mut base_config = HashMap::from([("bootstrap.servers", servers.as_str())]);
        if let Some(overrides) = config_overrides {
            base_config.extend(overrides);
        }

        for (k, v) in base_config.into_iter() {
            client_config.set(k, v);
        }
        if let Some(user) = kafka_user {
            if let Some(password) = kafka_password {
                client_config.set("security.protocol", "SASL_PLAINTEXT")
                    .set("sasl.mechanisms", "SCRAM-SHA-512")
                    .set("sasl.username", user)
                    .set("sasl.password", password);
            } else {
                panic!("password required for authentication");
            }
        }
        match client_type {
            KafkaClientType::Producer => {
                let producer: ThreadedProducer<DefaultProducerContext> =
                    client_config.create().expect("Failed to create producer");
                Self {
                    producer: Some(producer),
                    consumer: None,
                    topic,
                    partition_length
                }
            }
            KafkaClientType::Consumer => {
                let consumer: StreamConsumer =
                    client_config.create().expect("Failed to create consumer");
                Self {
                    producer: None,
                    consumer: Some(consumer),
                    topic,
                    partition_length
                }
            }
        }
    }
    pub fn publish_message(
        &self,
        key: &str,
        value: String,
        headers: Option<HashMap<String, String>>,
    ) {
        let record = BaseRecord::to(self.topic.as_str()).payload(&value).key(key);

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
    pub fn subscribe(&self) {

        // let topic = self.topic.as_str();
        //
        // let mut partition_list = TopicPartitionList::new();
        // for partition in 0..self.partition_length.unwrap() {
        //     println!("added partition = {}",partition);
        //     partition_list.add_partition(topic, (partition) as i32);
        //     partition_list.set_partition_offset(topic, (partition) as i32, Offset::End).unwrap();
        // }
        //
        // // This line is required for seek operation to be successful.
        //
        // &self.consumer.as_ref().unwrap().assign(&partition_list).unwrap();
        // for partition_list in partition_list.elements().iter() {
        //     let partition = partition_list.partition();
        //     println!("seeked partition = {}",partition);
        //     // info!("Fetching offset for partition: {}", partition);
        //     let (_low, high) = self.consumer.as_ref().unwrap().fetch_watermarks(topic,partition , Duration::from_secs(5)).unwrap();
        //     let offset = Offset::Offset(high);
        //
        //     // info!("Seeking on partition {} to offset: {:?}", partition, offset);
        //     &self.consumer.as_ref().unwrap().seek(topic, partition, offset, Duration::from_secs(5)).unwrap();
        // }

        let _ = &self
            .consumer
            .as_ref()
            .unwrap()
            .subscribe(&[self.topic.as_str()])
            .expect("unable to subscribe to topic");

    }

    pub fn unsubscribe(&self) {
        self.consumer.as_ref().unwrap().unsubscribe();
    }
    pub async fn consume_message(&mut self) -> BorrowedMessage {

        self.consumer
            .as_mut()
            .unwrap()
            .recv()
            .await
            .expect("message recv error")
    }
}
