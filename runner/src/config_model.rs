
struct ProducerConfig {
    rate: u32,
    in_topic: String,
    payload: serde_json::Value
}

enum Operator {
    EQUAL
}

struct Condition {
    operator: Operator,
    in_path: String,
    out_path: String
}

struct AnalyserConfig {
    out_topic: String,
    match_condition: Condition
}

struct Config {
    producer_config: ProducerConfig,
    analyser_config: AnalyserConfig
}