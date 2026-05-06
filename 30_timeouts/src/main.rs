// Timeouts are important for programs that connect to external resources.
// Go uses `select` with `time.After`; Rust uses `recv_timeout` on an mpsc
// receiver, which returns Err(RecvTimeoutError::Timeout) when the deadline
// is exceeded.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // First case: operation takes 2s, timeout is 1s — timeout fires.
    let (tx1, rx1) = mpsc::channel::<String>();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx1.send("result 1".to_string()).unwrap();
    });

    match rx1.recv_timeout(Duration::from_secs(1)) {
        Ok(res) => println!("{}", res),
        Err(_) => println!("timeout 1"),
    }

    // Second case: operation takes 2s, timeout is 3s — result arrives in time.
    let (tx2, rx2) = mpsc::channel::<String>();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx2.send("result 2".to_string()).unwrap();
    });

    match rx2.recv_timeout(Duration::from_secs(3)) {
        Ok(res) => println!("{}", res),
        Err(_) => println!("timeout 2"),
    }
}
