use std::collections::HashMap;
use rdkafka::Message;
use rdkafka::message::{BorrowedMessage, Header, OwnedHeaders, Headers};

pub fn get_message_headers(message: &BorrowedMessage) -> Option<HashMap<String, String>> {
    if let Some(headers) = message.headers() {
        let headers = (0..headers.count()).fold(HashMap::<String, String>::new(), |mut acc, i| {
            if let (k, Some(v)) = (headers.get(i).key, headers.get(i).value) {
                acc.insert(k.to_owned(), String::from_utf8_lossy(v).into_owned());
            }

            acc
        });

        return if headers.is_empty() {
            None
        } else {
            Some(headers)
        }
    }
    None
}

pub fn build_kafka_headers(headers: HashMap<String, String>) -> OwnedHeaders {
    let owned_headers = OwnedHeaders::new();

    let owned_headers = headers.iter().fold(owned_headers, |acc, x| {
        let header = Header { key: x.0, value: Some(x.1) };
        acc.insert(header)
    });

    owned_headers
}

pub fn get_raw_payload<'a>(message: &'a BorrowedMessage) -> &'a [u8] {
   let raw_payload = message.payload().expect("missing payload");
    return raw_payload
}

pub fn get_time_stamp(message: &BorrowedMessage) -> i64 {
     message.timestamp().to_millis().unwrap()
}