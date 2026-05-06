// Go's sync.WaitGroup is replaced in Rust by collecting JoinHandles into a Vec
// and calling .join() on each. This is more explicit than WaitGroup but
// achieves the same result: waiting for all spawned threads to complete.

use std::thread;
use std::time::Duration;

fn worker(id: i32) {
    println!("Worker {} starting", id);
    // Simulate an expensive task.
    thread::sleep(Duration::from_secs(1));
    println!("Worker {} done", id);
}

fn main() {
    let mut handles = Vec::new();

    // Launch 5 worker threads, equivalent to wg.Go(func() { worker(i) }).
    for i in 1..=5 {
        let handle = thread::spawn(move || {
            worker(i);
        });
        handles.push(handle);
    }

    // Wait for all workers to finish, equivalent to wg.Wait().
    for handle in handles {
        handle.join().unwrap();
    }
}
