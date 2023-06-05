use crate::consumer::models::{AnalyticalDataSet, View};
use std::collections::HashMap;

impl View {

    fn calculate_rate(&self, num_of_samples: f64, max_time: f64, min_time: f64) -> u64 {
        let rate =  (num_of_samples/ (max_time - min_time)) * 1000 as f64;
        rate.round() as u64
    }

    fn print_data_set_headers(&self) {
        println!("| P0 | P50 | P90 | P95 | P98 | P99 | P99.9 |messageCount|Throughput|");
        println!("|----|-----|-----|-----|-----|-----|-------|------------|----------|");
    }


    pub fn print_data_set(&self, data: &AnalyticalDataSet) {
        let message_rate = self.calculate_rate(data.message_count as f64, data.max_time as f64,data.min_time as f64 );
        println!();
        self.print_data_set_headers();
        print!("| {} ", data.latency.value_at_quantile(0.0));
        print!("| {} ", data.latency.value_at_quantile(0.50));
        print!("| {} ", data.latency.value_at_quantile(0.90));
        print!("| {} ", data.latency.value_at_quantile(0.95));
        print!("| {} ", data.latency.value_at_quantile(0.98));
        print!("| {} ", data.latency.value_at_quantile(0.99));
        print!("| {} ", data.latency.value_at_quantile(0.999));
        print!("| {} ", data.message_count);
        print!("| {} |", message_rate);
        println!();
        println!("================================================================");

    }


}

