use std::collections::HashMap;
use hdrhistogram::Histogram;

#[derive(Clone, Debug)]
pub struct View {
}

#[derive(Clone, Debug)]
pub struct AnalyticalDataSet {
    pub(crate) latency: Histogram::<u64>,
    pub(crate) max_time: i64,
    pub(crate) min_time: i64,
    pub(crate) message_count: u128,
}

impl AnalyticalDataSet {
    pub fn new() -> Self {
        AnalyticalDataSet {
            latency: Histogram::<u64>::new(2).unwrap(),
            max_time: i64::MIN,
            min_time: i64::MAX,
            message_count: 0
        }
    }
}