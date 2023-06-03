use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = 100)]
    pub rate: u32,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1024)]
    pub payload_size_in_bytes: u32,
}
