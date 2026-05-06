// Go's `select` statement waits on multiple channels simultaneously.
// Rust's std::sync::mpsc has no direct select primitive. We simulate it by
// merging two channels into one via a forwarding thread, then receiving
// in order from the merged channel.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create two channels, one for each simulated concurrent operation.
    let (tx_merged, rx_merged) = mpsc::channel::<String>();

    let tx1 = tx_merged.clone();
    let tx2 = tx_merged.clone();

    // Each thread sleeps to simulate a blocking operation, then sends.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx1.send("one".to_string()).unwrap();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx2.send("two".to_string()).unwrap();
    });

    // Drop the original merged sender so the channel closes after both
    // forwarding threads complete.
    drop(tx_merged);

    // Receive both messages in arrival order, equivalent to Go's select loop.
    for _ in 0..2 {
        let msg = rx_merged.recv().unwrap();
        println!("received {}", msg);
    }
}
