The first timer fires ~2s after the program starts. The second timer is stopped before it fires by dropping the receiver, causing the thread's send to fail silently. Go's `time.NewTimer` is modeled in Rust as a thread that sleeps then sends on a channel; `timer.Stop()` maps to dropping the receiver.
___
##### Run Command:

`$ cargo run`

##### Results:

`Timer 1 fired`
`Timer 2 stopped`
