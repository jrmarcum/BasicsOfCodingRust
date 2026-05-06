// Go's `context.Context` is used to carry cancellation signals across goroutines.
// Rust's equivalent is a shared cancellation flag using `Arc<AtomicBool>`,
// or a channel-based approach. This example shows an HTTP-like server that
// handles a slow request and can detect when the client cancels.
//
// We simulate the Go context example: a handler that waits 10 seconds,
// but can be cancelled early via a shared flag.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn hello_handler(cancelled: Arc<AtomicBool>) -> String {
    println!("server: hello handler started");

    // Simulate slow work: check for cancellation every 100ms for up to 10s.
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(100));
        if cancelled.load(Ordering::Relaxed) {
            println!("server: context canceled");
            println!("server: hello handler ended");
            return "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\ncontext canceled\n"
                .to_string();
        }
    }

    println!("server: hello handler ended");
    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nhello\n".to_string()
}

fn main() {
    let listener = TcpListener::bind("localhost:8090").unwrap();
    println!("Listening on localhost:8090");
    println!("curl localhost:8090/hello  (then Ctrl+C the curl to simulate cancellation)");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).unwrap_or(0);

        // Set a short read timeout so we can detect a dropped connection.
        stream
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();

        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_clone = cancelled.clone();

        // Spawn a thread to detect if the client disconnects (sends nothing / closes).
        let mut stream_clone = stream.try_clone().unwrap();
        thread::spawn(move || {
            let mut probe = [0u8; 1];
            // A read of 0 bytes or an error means the client closed the connection.
            loop {
                match stream_clone.read(&mut probe) {
                    Ok(0) | Err(_) => {
                        cancelled_clone.store(true, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => {}
                }
            }
        });

        let response = hello_handler(cancelled);
        let _ = stream.write_all(response.as_bytes());
    }
}
