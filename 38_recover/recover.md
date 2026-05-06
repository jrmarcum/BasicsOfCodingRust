`std::panic::catch_unwind` captures a panic and returns a `Result`, allowing execution to continue after the panic — equivalent to Go's `recover()` in a deferred function. Note: Go's output has a leading space before "a problem" (`" a problem"`); the Rust version prints it on a new line without a leading space.
___
##### Run Command:

`$ cargo run`

##### Results:

`Recovered. Error:`
`a problem`
`After safe_div()`
