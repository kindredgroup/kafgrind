
mod test_runner;
mod test_builder;

mod consumer;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::sleep;
use test_builder::{TestSuite};
use test_runner::TestRunner;

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::spawn;
use chrono::Utc;
use futures::executor::block_on;
use consumer::TestBuilderConsumer;
use time::OffsetDateTime;

pub fn run_producer_lib(rate_map: Vec<(u64, u64)>, sleep_between_run: u64) {
    let ts =  Arc::new(TestSuite::new(1000));
    let ts_clone = Arc::clone(&ts);
    let consumer_handle = spawn(move || {
        consumer_task(ts_clone, 0);
    });
    for (rate,time) in rate_map {
        let num_of_threads: u64 = 10;
       let mut threads = Vec::with_capacity(num_of_threads as usize);
        println!(" starting test for rate: {rate} msgs/sec for time: {time} mins on num_of_threads:{num_of_threads}");
        let test_id = format!("{rate}-{time}-{}", Utc::now());
        for thread_id in 0..num_of_threads {
            let thread_time_in_secs = time.clone() * 60;
            let rate_per_thread = rate / num_of_threads;
            let test_id_clone = test_id.clone();
            let test_suite = Arc::clone(&ts);
            threads.push(std::thread::spawn( move || {
                run_test(
                    Duration::from_secs(thread_time_in_secs),
                    rate_per_thread,
                     test_id_clone,
                     test_suite

                );
            }));
        }
        // Wait for all threads to finish.
        for handle in threads {
            handle.join().unwrap();
        }
        sleep(Duration::from_secs(sleep_between_run));
    }
}

pub fn run_test( total_run_time: Duration, target_rate_message_per_sec: u64, test_id: String, test_suite: Arc<TestSuite>) {
    producer_task(test_suite, total_run_time, target_rate_message_per_sec, test_id);
}
#[tokio::main]
async fn consumer_task(test_suite: Arc<TestSuite>, thread_id: u64) {
    let mut consumer = TestBuilderConsumer::new(thread_id);
    loop {

        if let Some(decision) = consumer.consume_decision().await {
            // println!("update snapshot");
            test_suite.update_store(decision.xid, decision.version, None);
            // match decision.decision {
            //     DecisionType::Committed => { test_suite.update_store(decision.xid, decision.version, Some(decision.version));},
            //     DecisionType::Aborted => { test_suite.update_store(decision.xid, decision.version, None);},
            // }

        }
     //   println!("in loop waiting for await");

    }
}

fn producer_task(test_suite: Arc<TestSuite>, total_run_time: Duration, target_rate_msg_per_sec: u64, test_id: String) {

    let started_at = OffsetDateTime::now_utc().unix_timestamp_nanos();
    let adjust_every = 100;
    let mut delay = Duration::from_nanos((1_000_000_000 as f64 / target_rate_msg_per_sec as f64) as u64);
    let mut done = 0;
    let mut skip = 0;
    let min_sleep = Duration::from_micros(500);
    let tr =TestRunner::new();
    let instant = Instant::now();
    while instant.elapsed() <= total_run_time {
        done += 1;
        if skip == 0 {
            thread::sleep(delay);
        } else {
            skip -= 1
        }

        let test_case = test_suite.get_test_case();
        tr.execute(test_case, test_id.clone());
        if done % adjust_every == 0 {
            let now = OffsetDateTime::now_utc().unix_timestamp_nanos();
            let elapsed = Duration::from_nanos((now - started_at) as u64).as_secs_f64();
            let effective_rate = done as f64 / elapsed;
            let scale = target_rate_msg_per_sec as f64 / effective_rate;
            let sleep = delay.as_secs_f64() / scale;
            if Duration::from_secs_f64(sleep) < min_sleep {
                skip = (min_sleep.as_secs_f64() / sleep).ceil() as i32 * adjust_every;
            } else {
                delay = Duration::from_secs_f64(sleep);
                skip = 0;
            }
        }



    }
}
