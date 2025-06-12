use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::VecDeque;

#[derive(Clone, Serialize)]
pub struct Metrics {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub total_latency: f64,
    pub min_latency: f64,
    pub max_latency: f64,
    pub requests_per_second: f64,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub recent_latencies: VecDeque<f64>,
    pub average_latency: f64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_latency: 0.0,
            min_latency: f64::MAX,
            max_latency: 0.0,
            requests_per_second: 0.0,
            start_time: None,
            end_time: None,
            recent_latencies: VecDeque::with_capacity(1000),
            average_latency: 0.0,
        }
    }

    pub fn record_request(&mut self, latency: f64, success: bool) {
        self.total_requests += 1;
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }

        self.total_latency += latency;
        self.min_latency = self.min_latency.min(latency);
        self.max_latency = self.max_latency.max(latency);

        self.recent_latencies.push_back(latency);
        if self.recent_latencies.len() > 1000 {
            self.recent_latencies.pop_front();
        }

        self.update_requests_per_second();
        self.update_average_latency();
    }

    fn update_requests_per_second(&mut self) {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            let duration = (end - start).num_seconds() as f64;
            if duration > 0.0 {
                self.requests_per_second = self.total_requests as f64 / duration;
            }
        }
    }

    fn update_average_latency(&mut self) {
        if self.total_requests > 0 {
            self.average_latency = self.total_latency / self.total_requests as f64;
        }
    }

    pub fn start_test(&mut self) {
        self.start_time = Some(Utc::now());
        self.end_time = None;
    }

    pub fn end_test(&mut self) {
        self.end_time = Some(Utc::now());
        self.update_requests_per_second();
    }
} 