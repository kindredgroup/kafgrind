use std::collections::{BTreeMap, HashMap};
use models::talos_models::MessageType::{Candidate, Decision};
use crate::runner::TestMessage;
use crate ::models::TestCase;
use crate::models::{AnalyticalDataSet, View};

impl AnalyticalDataSet {

    fn add_to_candidate_counter(&mut self) {
        self.candidate_message_count+=1;
    }
    fn add_to_decision_counter(&mut self) {
        self.decision_message_count+=1;
    }

    fn update_max_min_counters(&mut self, test_case: &TestCase) {
        if let Some(c) = test_case.candidate_publish_time {
            if  c > self.max_candidate_time {
                self.max_candidate_time = c;
            }

            if c < self.min_candidate_time {
                self.min_candidate_time = c;
            }
        }

        if let Some(c) = test_case.decision_publish_time {
            if  c > self.max_decision_time {
                self.max_decision_time = c;
            }

            if c < self.min_decision_time {
                self.min_decision_time = c;
            }
        }


    }

    fn add_to_histogram_if_complete (&mut self, test_case: &TestCase) {
        if test_case.candidate_publish_time.is_some() && test_case.decision_publish_time.is_some() {
            let d = test_case.decision_publish_time.unwrap();
            let c = test_case.candidate_publish_time.unwrap();
            // println!("d-c: {}", d - c);
            self.latency.record(<i64 as TryInto<u64>>::try_into(
                d - c).unwrap()
            ).unwrap();
        }
    }
}

pub struct Analyser {
    view: View,
    test_id_map: BTreeMap<String, AnalyticalDataSet>,
    records: HashMap<String, TestCase>,

}

impl Analyser {
    pub fn new(view: View) -> Self {
        Self {
            view,
            test_id_map: BTreeMap::new(),
            records: HashMap::new()
        }
    }
    fn get_record_by_id (&self, id: &str) -> Option<TestCase> {
        if let Some(record) = self.records.get(id) {
            Some(record.to_owned())
        } else {
            None
        }
    }

    pub fn handle_candidate(&mut self, id: String, test_case: TestCase) {

        if let Some(mut data_set) = self.test_id_map.get_mut(&*test_case.test_id) {
            // data set exists for same test id
            data_set.update_max_min_counters(&test_case);
            data_set.add_to_candidate_counter();
        } else {
            let mut data_set = AnalyticalDataSet::new();
            data_set.update_max_min_counters(&test_case);
            data_set.add_to_candidate_counter();
            self.test_id_map.insert(test_case.test_id.clone(), data_set);
        }
        self.records.insert(id, test_case);
    }

    pub fn handle_decision(&mut self, id: String, test_case: TestCase) {

        if let Some(mut data_set) = self.test_id_map.get_mut(&*test_case.test_id) {
            // data set exists for same test id
            data_set.add_to_histogram_if_complete(&test_case);
            data_set.update_max_min_counters(&test_case);
            data_set.add_to_decision_counter();
        } else {
            panic!(" Candidate must be present");
        }
    }

    pub fn update_data_set(&mut self, message: TestMessage) {
        if let Some(existing) = self.get_record_by_id(&message.id) {
            match message.message_type {
                Decision => {
                    let t = TestCase {
                        decision: message.decision,
                        decision_publish_time: Some(message.time),
                        ..existing
                    };
                    self.records.remove(&message.id);
                    if self.records.len() == 0 {
                        self.records = HashMap::new(); // Assign new one in case all decisions are received to free up memory
                    }
                    self.handle_decision(message.id.to_string(), t);
                },
                Candidate => {panic!("Candidate already exist");}
                _ => {}
            }

        } else {
            // Ignoring if decision comes and there is no entry for candidate. May be it's from previous test run.
            if message.message_type == Candidate {
                let test_id = message.test_id.expect("test id missing on candidate message");
                let t = TestCase {
                    decision: None,
                    test_id,
                    candidate_publish_time: Some(message.time),
                    decision_publish_time: None
                };
                self.handle_candidate(message.id.to_string(), t);
            }
        }
    }

    pub fn update_view(&mut self) {
        for (test_id, data_set) in &self.test_id_map {
            self.view.print_data_set(data_set, test_id);
            for v in data_set.latency.iter_recorded() {
                println!("{}'th percentile of data is {} with {} samples",
                         v.percentile(), v.value_iterated_to(), v.count_at_value());
            }
        }
        println!("self.records.length: {}", self.records.len());

        println!("==================================================================");

    }

}
