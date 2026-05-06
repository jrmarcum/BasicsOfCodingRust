// For shared mutable state across threads, Rust uses Arc<Mutex<T>>.
// Arc (Atomic Reference Counting) replaces Go's implicit sharing;
// Mutex<T> wraps the data and requires locking before access.
// This is equivalent to Go's sync.Mutex protecting a map.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    counters: Mutex<HashMap<String, i32>>,
}

impl Container {
    fn inc(&self, name: &str) {
        // Lock the mutex before accessing counters; the guard is released
        // automatically when it goes out of scope (RAII, like Go's defer Unlock).
        let mut counters = self.counters.lock().unwrap();
        *counters.entry(name.to_string()).or_insert(0) += 1;
    }
}

fn main() {
    let c = Arc::new(Container {
        counters: Mutex::new(HashMap::from([
            ("a".to_string(), 0),
            ("b".to_string(), 0),
        ])),
    });

    let do_increment = |name: &'static str, n: i32, container: Arc<Container>| {
        move || {
            for _ in 0..n {
                container.inc(name);
            }
        }
    };

    // Spawn three threads: two increment "a", one increments "b".
    let c1 = Arc::clone(&c);
    let c2 = Arc::clone(&c);
    let c3 = Arc::clone(&c);

    let h1 = thread::spawn(do_increment("a", 10000, c1));
    let h2 = thread::spawn(do_increment("a", 10000, c2));
    let h3 = thread::spawn(do_increment("b", 10000, c3));

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();

    // Print the final counter values. Rust's HashMap Debug format differs
    // from Go's: {"a": 20000, "b": 10000} vs map[a:20000 b:10000].
    let counters = c.counters.lock().unwrap();
    println!("{:?}", *counters);
}
