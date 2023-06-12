use std::prelude::rust_2024::*;
use chrono::{NaiveDateTime, DateTime, Utc, Duration};
use std::sync::SgxMutex;

/// Creates a time function that increments by one second each time.

struct IncreasingTime {
    times: std::ops::RangeFrom<i64>,
}

impl IncreasingTime {
    fn new() -> Self {
        Self { times: (0..) }
    }
    fn next(&mut self) -> DateTime<Utc> {
        let next_value = self.times.next().unwrap();
        let naive = NaiveDateTime::from_timestamp(next_value, 0);
        DateTime::<Utc>::from_utc(naive, Utc)
    }
}

pub fn mock_time_fn() -> DateTime<Utc> {
    let timer = SgxMutex::new(IncreasingTime::new());
    let x = timer.lock().unwrap().next();
    x
}

pub fn mock_time_fn_with_delay(duration: Duration) -> DateTime<Utc> {
    let timer = SgxMutex::new(IncreasingTime::new());
    let mut timer = timer.lock().unwrap();
    let current_time = timer.next();
    let new_time = current_time + duration;
    new_time
}
