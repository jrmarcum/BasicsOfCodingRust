In Rust, errors are communicated via the `Result<T, E>` type rather than separate return values as in Go. Custom error types implement `std::fmt::Display` and `std::error::Error`. Rust's `match` or `if let` replaces Go's `if r, e := f(i); e != nil` idiom.
___
##### Run Command:

`$ cargo run`

##### Results:

`f1 worked: 10`
`f1 failed: can't work with 42`
`f2 worked: 10`
`f2 failed: 42 - can't work with it`
`42`
`can't work with it`
