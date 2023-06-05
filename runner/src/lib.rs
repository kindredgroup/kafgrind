use std::fs;

pub mod config_model;
use config_model::Config;

pub fn file_to_config() {
    let file = fs::File::open("kafgrind_config.json")
        .expect("config.json file missing or cannot be opened.");
    let config: Config =
        serde_json::from_reader(file).expect("config.json file should be proper JSON");
    println!("{config:?}");
}
