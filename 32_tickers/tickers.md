When we run this program the ticker ticks ~3 times before we stop it. Actual timestamps reflect the real run time. Go's `time.NewTicker` is modeled in Rust as a thread that loops with `thread::sleep` and sends on a channel; dropping the receiver signals the ticker thread to exit. Note: due to thread scheduling, tick output order may vary slightly.
___
##### Run Command:

`$ cargo run`

##### Results:

`Tick at Instant { ... }`
`Tick at Instant { ... }`
`Tick at Instant { ... }`
`Ticker stopped`
