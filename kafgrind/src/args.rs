use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "kafgrind")]
#[command(author = "Anil Sharma")]
#[command(version = "0.0.1")]
#[command(about = "Simple to use Kafka echo benchmarking tool", long_about = " This tool tries to produce with mentioned size messages at target rate. In separate thread it runs consumer and analyser to read the messages to output latency and throughput figures. ")]
pub struct Args {
    /// target throughput rate
    #[arg(short, long)]
    pub rate: u64,

    /// payload size in bytes
    #[arg(short, long)]
    pub size_in_bytes: usize,

    /// payload size in bytes
    #[arg(short, long)]
    pub number_of_records: u64,

    /// brokers urls separated by `,`
    #[arg(short, long)]
    pub brokers: String,

    /// brokers urls separated by `,`
    #[arg(short, long)]
    pub topic: String,

    /// kafka user
    #[arg(short, long)]
    pub user: Option<String>,

    /// kafka password
    #[arg(short, long)]
    pub password: Option<String>,

}
