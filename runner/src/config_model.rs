use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProducerConfig {
    pub rate: u32,
    pub in_topic: String,
    pub payload: serde_json::Value,
}

pub enum Operator {
    EQUAL,
}

pub struct Condition {
    pub operator: Operator,
    pub in_path: String,
    pub out_path: String,
}

pub struct AnalyserConfig {
    pub out_topic: String,
    pub match_condition: Condition,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub producer_config: ProducerConfig,
    // analyser_config: AnalyserConfig
}
