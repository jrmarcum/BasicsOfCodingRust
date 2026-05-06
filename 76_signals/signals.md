#### When we run this program it will block waiting for a signal. By typing `Ctrl+C` (which the terminal shows as `^C`) we can send a `SIGINT` signal, causing the program to print a message and exit.
___
##### Run Command:

`$ cargo run`

##### Results:

`awaiting signal`
`^C`
`exiting`
___
#### Note: Rust uses the `ctrlc` crate for cross-platform signal handling (Windows and Unix). Go's modern approach uses `signal.NotifyContext` which integrates with `context.Context` and reports `context.Cause(ctx)` as the cancellation reason. The Rust `ctrlc` crate uses an `mpsc::channel` to communicate the signal from the handler thread to the main thread — similar in concept to Go's channel-based `<-ctx.Done()` wait.
