use std::collections::HashMap;
use std::io::Stdout;
use hdrhistogram::Histogram;
use tokio::sync::mpsc;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use models::talos_models::DecisionType;
#[derive(Clone, Debug)]
pub struct TestCase {
    pub decision: Option<DecisionType>,
    pub candidate_publish_time: Option<i64>,
    pub decision_publish_time: Option<i64>,
    pub test_id: String
}

impl TestCase {
    pub fn new(test_id: String) -> Self {
        TestCase {
            decision: None,
            candidate_publish_time: None,
            decision_publish_time: None,
            test_id
        }
    }
}
#[derive(Clone, Debug)]
pub struct View {
}

#[derive(Clone, Debug)]
pub struct AnalyticalDataSet {
    pub(crate) latency: Histogram::<u64>,
    pub(crate) max_candidate_time: i64,
    pub(crate) min_candidate_time: i64,
    pub(crate) max_decision_time: i64,
    pub(crate) min_decision_time: i64,
    pub(crate) candidate_message_count: u64,
    pub(crate) decision_message_count: u64,
}

impl AnalyticalDataSet {
    pub fn new() -> Self {
        AnalyticalDataSet {
            latency: Histogram::<u64>::new(2).unwrap(),
            max_decision_time: i64::MIN,
            max_candidate_time: i64::MIN,
            min_decision_time: i64::MAX,
            min_candidate_time: i64::MAX,
            candidate_message_count: 0,
            decision_message_count: 0
        }
    }
}