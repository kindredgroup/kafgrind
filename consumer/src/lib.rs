extern crate core;

use crate::analyser::Analyser;
use crate::models::View;
use crate::runner::Runner;

pub mod runner;
pub mod analyser;
pub mod utils;
pub mod models;
pub mod view;

pub async fn run_consumer() {
    let view = View{};
    let analyser = Analyser::new(view);
    let mut runner = Runner::new(analyser);
    runner.run().await;
}
