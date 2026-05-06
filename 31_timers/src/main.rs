// Go's time.NewTimer is simulated in Rust using a thread that sleeps and then
// sends on a channel. Stopping a timer is simulated by dropping the sender
// before the thread wakes, causing the send to fail silently.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Timer 1: fires after 2 seconds.
    let (tx1, rx1) = mpsc::channel::<()>();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        // If the receiver was dropped, this send fails but we ignore it.
        let _ = tx1.send(());
    });

    // Block until timer 1 fires, analogous to `<-timer1.C`.
    rx1.recv().unwrap();
    println!("Timer 1 fired");

    // Timer 2: we stop it before it fires by dropping the receiver.
    let (tx2, rx2) = mpsc::channel::<()>();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        // The receiver is already dropped; this send returns an error.
        let _ = tx2.send(());
        // Without an active receiver the "Timer 2 fired" message never prints.
    });

    // Drop the receiver immediately, cancelling timer 2.
    drop(rx2);
    println!("Timer 2 stopped");

    // Wait long enough to confirm timer 2 never fires.
    thread::sleep(Duration::from_secs(2));
}
