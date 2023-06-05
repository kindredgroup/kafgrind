use consumer::run_consumer;

#[tokio::main]
async fn main() {
    run_consumer("localhost:9092".to_string(), "test.topic".to_string(), 10, None, None).await;
}
