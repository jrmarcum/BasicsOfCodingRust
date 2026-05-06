Running the program shows the counters updated as expected. Rust uses `Arc<Mutex<T>>` to share mutable state across threads: `Arc` provides shared ownership and `Mutex` ensures exclusive access. The mutex guard is released automatically via RAII (equivalent to Go's `defer mu.Unlock()`). Note: Rust's `HashMap` debug format is `{"a": 20000, "b": 10000}` while Go prints `map[a:20000 b:10000]`. HashMap iteration order is non-deterministic so key order may vary.
___
##### Run Command:

`$ cargo run`

##### Results:

`{"a": 20000, "b": 10000}`
