// Metrics collection utilities

use std::time::Instant;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static::lazy_static! {
    static ref METRICS: RwLock<HashMap<String, u64>> = RwLock::new(HashMap::new());
}

pub struct Timer {
    start: Instant,
    metric_name: String,
}

impl Timer {
    pub fn new(metric_name: String) -> Self {
        Self {
            start: Instant::now(),
            metric_name,
        }
    }

    pub fn stop(self) {
        let duration = self.start.elapsed().as_millis() as u64;
        let mut metrics = METRICS.write().unwrap();
        *metrics.entry(self.metric_name).or_insert(0) += duration;
    }
}

pub fn record_metric(metric_name: &str, value: u64) {
    let mut metrics = METRICS.write().unwrap();
    *metrics.entry(metric_name.to_string()).or_insert(0) += value;
}

pub fn get_metrics() -> HashMap<String, u64> {
    let metrics = METRICS.read().unwrap();
    metrics.clone()
}

pub fn reset_metrics() {
    let mut metrics = METRICS.write().unwrap();
    metrics.clear();
}