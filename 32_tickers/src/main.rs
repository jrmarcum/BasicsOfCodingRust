// Go's time.NewTicker is simulated in Rust with a thread that loops with
// thread::sleep and sends tick timestamps on a channel. Stopping the ticker
// is done by dropping the receiver; the ticker thread detects the broken
// channel via SendError and exits its loop.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let (tx, rx) = mpsc::channel::<Instant>();

    // Ticker thread: sends a tick every 500ms until the receiver is dropped.
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));
            if tx.send(Instant::now()).is_err() {
                // Receiver was dropped — stop ticking.
                break;
            }
        }
    });

    // Receive ticks for ~1600ms (should yield ~3 ticks), then stop.
    let deadline = Instant::now() + Duration::from_millis(1600);
    loop {
        match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok(t) => println!("Tick at {:?}", t),
            Err(_) => break,
        }
    }

    // Dropping rx here signals the ticker thread to exit.
    drop(rx);
    println!("Ticker stopped");
}
