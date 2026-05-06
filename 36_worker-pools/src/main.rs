// A worker pool uses mpsc channels for both job distribution and result
// collection. Workers receive jobs from a shared channel and send results
// back on a results channel — the same pattern as Go's goroutine pool.
//
// Because mpsc::Receiver is single-consumer, we wrap it in Arc<Mutex<>>
// so multiple workers can take turns receiving jobs.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    const NUM_JOBS: i32 = 5;

    let (job_tx, job_rx) = mpsc::channel::<i32>();
    let (result_tx, result_rx) = mpsc::channel::<i32>();

    // Share the receiver across workers.
    let job_rx = Arc::new(Mutex::new(job_rx));

    // Start 3 worker threads.
    let mut handles = Vec::new();
    for w in 1..=3 {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();
        let handle = thread::spawn(move || {
            loop {
                // Acquire the lock, take one job, then immediately release the lock
                // so other workers can proceed. Using a block here ensures the
                // MutexGuard is dropped before the thread sleeps.
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                    // MutexGuard dropped here
                };
                match job {
                    Ok(j) => {
                        println!("worker {} started  job {}", w, j);
                        // Simulate expensive work without holding the lock.
                        thread::sleep(Duration::from_secs(1));
                        println!("worker {} finished job {}", w, j);
                        result_tx.send(j * 2).unwrap();
                    }
                    Err(_) => break, // sender dropped — no more jobs
                }
            }
        });
        handles.push(handle);
    }

    // Send 5 jobs, then drop the sender to close the channel.
    for j in 1..=NUM_JOBS {
        job_tx.send(j).unwrap();
    }
    drop(job_tx);

    // Drop the cloned sender so result_rx closes after all workers finish.
    drop(result_tx);

    // Collect all results (blocks until each arrives).
    for _ in 1..=NUM_JOBS {
        result_rx.recv().unwrap();
    }

    // Join all worker threads.
    for handle in handles {
        handle.join().unwrap();
    }
}
