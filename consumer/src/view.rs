use crate::models::{AnalyticalDataSet, View};
use std::collections::HashMap;

impl View {

    fn calculate_rate(&self, num_of_samples: f64, max_time: f64, min_time: f64) -> u64 {
        let rate =  (num_of_samples/ (max_time - min_time)) * 1000 as f64;
        rate.round() as u64
    }

    fn print_data_set_headers(&self) {
        println!("| P0 | P50 | P90 | P95 | P98 | P99 | P99.9 |CandidateCount|DecisionCount|CandidateRate|DecisionRate|");
        println!("|----|-----|-----|-----|-----|-----|-------|--------------|-------------|-------------|------------|");
    }


    pub fn print_data_set(&self, data: &AnalyticalDataSet, test_id: &str) {
        let candidate_rate = self.calculate_rate(data.candidate_message_count as f64, data.max_candidate_time as f64,data.min_candidate_time as f64 );
        let decision_rate = self.calculate_rate(data.decision_message_count as f64, data.max_decision_time as f64,data.min_decision_time as f64 );
        println!("TestId: {test_id}");
        println!();
        self.print_data_set_headers();
        print!("| {} ", data.latency.value_at_quantile(0.0));
        print!("| {} ", data.latency.value_at_quantile(0.50));
        print!("| {} ", data.latency.value_at_quantile(0.90));
        print!("| {} ", data.latency.value_at_quantile(0.95));
        print!("| {} ", data.latency.value_at_quantile(0.98));
        print!("| {} ", data.latency.value_at_quantile(0.99));
        print!("| {} ", data.latency.value_at_quantile(0.999));
        print!("| {} ", data.candidate_message_count);
        print!("| {} ", data.decision_message_count);
        print!("| {} ", candidate_rate);
        println!("| {} |", decision_rate);

    }


}

