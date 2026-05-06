We expect exactly 50,000 operations. Using a non-atomic integer with `+= 1` across threads would cause data races (which Rust's type system prevents at compile time). `AtomicU64::fetch_add` replaces Go's `atomic.Uint64.Add`; `Arc<AtomicU64>` replaces Go's pointer-based sharing.
___
##### Run Command:

`$ cargo run`

##### Results:

`ops: 50000`
