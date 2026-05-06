// Rust's std::sync::atomic provides atomic types for lock-free concurrent access.
// AtomicU64::fetch_add replaces Go's atomic.Uint64.Add.
// Arc<AtomicU64> allows sharing the counter across threads, replacing Go's
// implicit sharing via goroutine closures.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // Wrap the atomic counter in an Arc for shared ownership across threads.
    let ops = Arc::new(AtomicU64::new(0));

    let mut handles = Vec::new();

    // Spawn 50 threads that each increment the counter 1000 times.
    for _ in 0..50 {
        let ops_clone = Arc::clone(&ops);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // fetch_add is the atomic equivalent of Go's ops.Add(1).
                ops_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish, equivalent to wg.Wait().
    for handle in handles {
        handle.join().unwrap();
    }

    // Load the final value atomically.
    println!("ops: {}", ops.load(Ordering::Relaxed));
}
