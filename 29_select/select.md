We receive the values `"one"` and then `"two"` as expected. The total execution time is only ~2 seconds since both sleeps run concurrently. Rust's `std::sync::mpsc` has no built-in `select`; the pattern here merges two senders into one channel so messages arrive in completion order, matching Go's `select` behavior.
___
##### Run Command:

`$ cargo run`

##### Results:

`received one`
`received two`
