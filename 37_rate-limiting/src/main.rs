// Rate limiting controls resource utilization and maintains quality of service.
// Go uses a ticker channel as a limiter. In Rust we use thread::sleep between
// requests for basic limiting, and a pre-filled channel for bursty limiting.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    // Basic rate limiting: handle one request every 200ms.
    let requests: Vec<i32> = (1..=5).collect();

    for req in &requests {
        thread::sleep(Duration::from_millis(200));
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        println!("request {} {:.9}", req, now.as_secs_f64());
    }

    // Bursty rate limiting: allow up to 3 immediate requests, then throttle.
    // Pre-fill a channel with 3 tokens to represent the burst allowance.
    let (burst_tx, burst_rx) = mpsc::sync_channel::<()>(3);
    for _ in 0..3 {
        burst_tx.send(()).unwrap();
    }

    // Refill one token every 200ms in a background thread.
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(200));
        if burst_tx.send(()).is_err() {
            break;
        }
    });

    let bursty_requests: Vec<i32> = (1..=5).collect();
    for req in &bursty_requests {
        // Consume one token before handling each request.
        burst_rx.recv().unwrap();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        println!("request {} {:.9}", req, now.as_secs_f64());
    }
}
