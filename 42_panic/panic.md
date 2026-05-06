#### Running this program will cause it to panic, print an error message, and exit with a non-zero status.
___
##### Run Command:

`$ cargo run`

##### Results:

`thread 'main' panicked at 'a problem', src/main.rs:10:5`
`note: run with RUST_BACKTRACE=1 environment variable to display a backtrace`
___
#### Note: Rust's panic output format differs from Go's. Go prints `panic: a problem` followed by goroutine traces; Rust prints the panic message with file/line info to stderr. Only stderr output is shown in Results above. Unlike Go which uses exceptions in some contexts, Rust uses `Result<T, E>` for recoverable errors and `panic!` only for truly unrecoverable situations.
