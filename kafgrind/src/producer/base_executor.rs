use crate::producer::models::Executor;
use crate::kafka::kafka_client::KafkaClient;
use std::collections::HashMap;
use std::time::SystemTime;

/// Sends a same message each instance
pub struct BaseExecutor {
    kafka_client: KafkaClient,
    message: String,
}

pub fn get_message_type_hashmap() -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::default();
    let publish_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    headers.insert(
        "publish_timestamp".to_owned(),
        publish_timestamp.to_string()

    );
    headers
}

impl BaseExecutor {
    pub fn new(kafka_client: KafkaClient, message: String) -> Self {
        Self {
            kafka_client,
            message,
        }
    }
}

impl Executor for BaseExecutor {
    fn execute(&self) {
        let headers: HashMap<String, String> = get_message_type_hashmap();
        let key = headers.get("publish_timestamp").unwrap();
        self.kafka_client.publish_message(
            &*key,
            self.message.clone(),
            Some(headers.clone()),
        );
    }
}
