// Go channels map to Rust's std::sync::mpsc (multi-producer, single-consumer).
// `make(chan string)` becomes `mpsc::channel::<String>()`.
// The channel <- send syntax becomes `tx.send(value).unwrap()`.
// The <- receive syntax becomes `rx.recv().unwrap()`.

use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a new channel. tx is the sender, rx is the receiver.
    let (tx, rx) = mpsc::channel();

    // Spawn a thread that sends "ping" on the channel,
    // analogous to Go's `go func() { messages <- "ping" }()`.
    thread::spawn(move || {
        tx.send("ping".to_string()).unwrap();
    });

    // Receive the value from the channel. This blocks until a value arrives,
    // matching Go's default unbuffered channel behavior.
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}
