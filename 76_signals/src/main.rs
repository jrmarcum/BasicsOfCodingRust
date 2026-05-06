// Sometimes we'd like our Rust programs to intelligently handle OS signals.
// For example, we might want a server to gracefully shut down when it receives
// a `SIGINT` (Ctrl+C). The `ctrlc` crate provides cross-platform signal handling,
// mirroring Go's `signal.NotifyContext` approach.

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Register a Ctrl+C (SIGINT) handler. On Unix this also catches SIGTERM.
    // The handler sends a message on the channel to unblock the main thread.
    ctrlc::set_handler(move || {
        tx.send(()).expect("Could not send signal on channel");
    })
    .expect("Error setting Ctrl-C handler");

    // The program will wait here until the signal is received.
    println!("awaiting signal");
    rx.recv().expect("Could not receive from channel");

    println!();
    println!("exiting");
}
