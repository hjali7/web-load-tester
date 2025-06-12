use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use warp::Filter;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestConfig {
    url: String,
    concurrent_users: usize,
    duration_seconds: u64,
    ramp_up_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestMetrics {
    total_requests: usize,
    successful_requests: usize,
    failed_requests: usize,
    average_latency: f64,
    min_latency: f64,
    max_latency: f64,
    requests_per_second: f64,
    recent_latencies: Vec<f64>,
}

struct TestState {
    metrics: TestMetrics,
    is_running: bool,
}

impl TestState {
    fn new() -> Self {
        TestState {
            metrics: TestMetrics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_latency: 0.0,
                min_latency: f64::MAX,
                max_latency: 0.0,
                requests_per_second: 0.0,
                recent_latencies: Vec::new(),
            },
            is_running: false,
        }
    }
}

#[tokio::main]
async fn main() {
    println!("{}", "Starting Web Load Tester...".green().bold());
    
    let state = Arc::new(Mutex::new(TestState::new()));
    let state_filter = warp::any().map(move || Arc::clone(&state));

    // API routes
    let start_route = warp::path("api")
        .and(warp::path("start"))
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(start_test);

    let metrics_route = warp::path("api")
        .and(warp::path("metrics"))
        .and(warp::get())
        .and(state_filter)
        .and_then(get_metrics);

    // Serve static files
    let static_files = warp::path("static")
        .and(warp::fs::dir("static"));

    let routes = start_route
        .or(metrics_route)
        .or(static_files)
        .or(warp::fs::file("static/index.html"));

    println!("{}", "Server running at http://localhost:3000".blue().bold());
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

async fn start_test(
    config: TestConfig,
    state: Arc<Mutex<TestState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{}", "Starting new load test...".yellow().bold());
    println!("{} {}", "Target URL:".cyan(), config.url);
    println!("{} {}", "Concurrent Users:".cyan(), config.concurrent_users);
    println!("{} {}", "Duration:".cyan(), format!("{} seconds", config.duration_seconds));
    println!("{} {}", "Ramp-up Time:".cyan(), format!("{} seconds", config.ramp_up_seconds));

    let mut state_guard = state.lock().await;
    state_guard.is_running = true;
    state_guard.metrics = TestMetrics {
        total_requests: 0,
        successful_requests: 0,
        failed_requests: 0,
        average_latency: 0.0,
        min_latency: f64::MAX,
        max_latency: 0.0,
        requests_per_second: 0.0,
        recent_latencies: Vec::new(),
    };
    drop(state_guard);

    let client = Client::new();
    let start_time = Instant::now();
    let end_time = start_time + Duration::from_secs(config.duration_seconds);
    let ramp_up_duration = Duration::from_secs(config.ramp_up_seconds);

    let mut handles = Vec::new();
    for i in 0..config.concurrent_users {
        let client = client.clone();
        let url = config.url.clone();
        let state = Arc::clone(&state);
        let delay = (ramp_up_duration.as_millis() as f64 * (i as f64 / config.concurrent_users as f64)) as u64;
        
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            while Instant::now() < end_time {
                let request_start = Instant::now();
                match client.get(&url).send().await {
                    Ok(response) => {
                        let latency = request_start.elapsed().as_secs_f64() * 1000.0;
                        let mut state_guard = state.lock().await;
                        state_guard.metrics.total_requests += 1;
                        if response.status().is_success() {
                            state_guard.metrics.successful_requests += 1;
                            println!("{} {} {} {}", 
                                "Request".green(), 
                                format!("#{}", state_guard.metrics.total_requests).white(),
                                "completed in".green(),
                                format!("{:.2}ms", latency).green()
                            );
                        } else {
                            state_guard.metrics.failed_requests += 1;
                            println!("{} {} {} {} {}", 
                                "Request".red(), 
                                format!("#{}", state_guard.metrics.total_requests).white(),
                                "failed with status".red(),
                                response.status().as_str().red(),
                                format!("({:.2}ms)", latency).red()
                            );
                        }
                        state_guard.metrics.recent_latencies.push(latency);
                        if state_guard.metrics.recent_latencies.len() > 100 {
                            state_guard.metrics.recent_latencies.remove(0);
                        }
                        state_guard.metrics.min_latency = state_guard.metrics.min_latency.min(latency);
                        state_guard.metrics.max_latency = state_guard.metrics.max_latency.max(latency);
                        state_guard.metrics.average_latency = state_guard.metrics.recent_latencies.iter().sum::<f64>()
                            / state_guard.metrics.recent_latencies.len() as f64;
                        state_guard.metrics.requests_per_second = state_guard.metrics.total_requests as f64
                            / (Instant::now() - start_time).as_secs_f64();
                    }
                    Err(e) => {
                        let mut state_guard = state.lock().await;
                        state_guard.metrics.total_requests += 1;
                        state_guard.metrics.failed_requests += 1;
                        println!("{} {} {} {}", 
                            "Request".red(), 
                            format!("#{}", state_guard.metrics.total_requests).white(),
                            "failed with error:".red(),
                            e.to_string().red()
                        );
                    }
                }
                sleep(Duration::from_millis(100)).await;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let mut state_guard = state.lock().await;
    state_guard.is_running = false;
    
    println!("{}", "\nTest completed!".green().bold());
    println!("{}", "Final Results:".cyan().bold());
    println!("{} {}", "Total Requests:".cyan(), state_guard.metrics.total_requests.to_string().white());
    println!("{} {}", "Successful Requests:".green(), state_guard.metrics.successful_requests.to_string().white());
    println!("{} {}", "Failed Requests:".red(), state_guard.metrics.failed_requests.to_string().white());
    println!("{} {}", "Average Latency:".cyan(), format!("{:.2}ms", state_guard.metrics.average_latency).white());
    println!("{} {}", "Min Latency:".cyan(), format!("{:.2}ms", state_guard.metrics.min_latency).white());
    println!("{} {}", "Max Latency:".cyan(), format!("{:.2}ms", state_guard.metrics.max_latency).white());
    println!("{} {}", "Requests/Second:".cyan(), format!("{:.2}", state_guard.metrics.requests_per_second).white());

    Ok(warp::reply::json(&state_guard.metrics))
}

async fn get_metrics(
    state: Arc<Mutex<TestState>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let state_guard = state.lock().await;
    Ok(warp::reply::json(&state_guard.metrics))
} 