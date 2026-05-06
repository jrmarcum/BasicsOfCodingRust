Running this program shows the first operation timing out and the second succeeding. `recv_timeout(Duration)` replaces Go's `select` with `time.After`: it returns `Err(Timeout)` when the deadline expires, or `Ok(value)` when the sender delivers before the timeout.
___
##### Run Command:

`$ cargo run`

##### Results:

`timeout 1`
`result 2`
