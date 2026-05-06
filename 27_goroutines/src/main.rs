// Go goroutines map to Rust threads. `go f(s)` becomes `thread::spawn(|| f(s))`.
// Unlike Go's `time.Sleep` workaround, Rust joins handles explicitly to wait
// for threads to finish — a more robust approach than sleeping.

use std::thread;

fn f(from: &str) {
    for i in 0..3 {
        println!("{} : {}", from, i);
    }
}

fn main() {
    // Direct synchronous call — same as Go.
    f("direct");

    // Spawn a thread for f("goroutine"), analogous to `go f("goroutine")`.
    // The string must be owned since the closure moves into the thread.
    let handle = thread::spawn(|| {
        f("goroutine");
    });

    // Spawn a thread for an anonymous function, analogous to Go's
    // `go func(msg string) { fmt.Println(msg) }("going")`.
    let handle2 = thread::spawn(|| {
        println!("going");
    });

    // Join both threads to wait for them to finish.
    handle.join().unwrap();
    handle2.join().unwrap();

    println!("done");
}
