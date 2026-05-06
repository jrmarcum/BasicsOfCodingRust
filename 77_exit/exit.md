#### `std::process::exit` terminates the program immediately with the given status code. No output is printed to stdout — only the exit code is observable.
___
##### Run Command:

`$ cargo run ; echo "exit status: $?"` (Unix/bash)

`$ cargo run; echo "Exit status: $LASTEXITCODE"` (Windows PowerShell)

##### Results:

`exit status: 3`
___
#### Note: Unlike normal function returns, `std::process::exit` terminates immediately. In Go, `defer` statements do NOT run when `os.Exit` is called — that's why the `fmt.Println("!")` in the Go example never prints. In Rust, `Drop` implementations for stack-allocated variables ARE still called on `process::exit` (unlike Go's defers), but any code placed *after* the `exit` call is simply unreachable. The program produces no stdout output and exits with code 3. Unlike Go, Rust's `main` can also return a `Result` or an `ExitCode` to signal failure without `process::exit`.
