use std::collections::{BTreeMap, HashMap};
use crate::consumer::runner::TestMessage;
use crate::consumer::models::{AnalyticalDataSet, View};

impl AnalyticalDataSet {

    fn add_to_message_counter(&mut self) {
        self.message_count+=1;
    }

    fn update_max_min_counters(&mut self, kafka_time: i64) {
        if  kafka_time > self.max_time {
            self.max_time = kafka_time;
        }

        if kafka_time < self.min_time {
            self.min_time = kafka_time;
        }
    }

    fn add_to_histogram (&mut self, test_message: &TestMessage) {
        self.latency.record((test_message.received_timestamp - test_message.publish_timestamp) as u64).unwrap();

    }
}

pub struct Analyser {
    view: View,
    data_set: AnalyticalDataSet
}

impl Analyser {
    pub fn new(view: View) -> Self {
        Self {
            view,
            data_set: AnalyticalDataSet::new(),
        }
    }



    pub fn update_data_set(&mut self, message: TestMessage) {
        self.data_set.add_to_message_counter();
        self.data_set.update_max_min_counters(message.kafka_time_stamp);
        self.data_set.add_to_histogram(&message);
    }

    pub fn update_view(&mut self) {

        self.view.print_data_set(&self.data_set);


    }

}
