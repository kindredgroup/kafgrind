use std::time::Duration;
use clap::Parser;
use kafgrind::args::Args;
use kafgrind::generate_payload;
use kafgrind::producer::run_base_producer_task;
use kafgrind::consumer::run_consumer;
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let rate = args.rate;
    let brokers = args.brokers;
    let topic = args.topic;
    let message = generate_payload(args.size_in_bytes);
    let number_of_records = args.number_of_records;
    let kafka_user = args.user;
    let kafka_password = args.password;
    let brokers_clone = brokers.clone();
    let kafka_user_clone = kafka_user.clone();
    let kafka_password_clone = kafka_password.clone();
    let topic_clone = topic.clone();
    let consumer_run_handle = tokio::spawn(async move {
        run_consumer(number_of_records,brokers.clone(), topic.clone(), 10, kafka_user.clone(), kafka_password.clone()).await;
    });
    println!("sleeping for 60 secs. To give consumer group enough time to be stable on broker.");
    std::thread::sleep(Duration::from_secs(60));
    println!("Starting producer threads.");
    let producer_handler = std::thread::spawn (move ||{
        run_base_producer_task(rate, number_of_records as u64, brokers_clone, topic_clone, message.clone(), kafka_user_clone, kafka_password_clone  );
    });
    futures::future::join_all([consumer_run_handle]).await;
    // producer_handler.join().expect("unable to join producer thread");
}
