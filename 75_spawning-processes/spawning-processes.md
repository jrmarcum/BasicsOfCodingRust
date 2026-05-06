#### The spawned programs return output that is the same as if we had run them directly from the command-line. Note: this example uses platform-specific commands (`cmd /C` on Windows, `date`/`grep`/`bash` on Unix).
___
##### Run Command:

`$ cargo run`

##### Results:

`> date`
`Wed 05/06/2026`
`command exit rc = 1`
`> grep hello`
`hello grep`
`> dir`
` Volume in drive D is Data`
`...`
___
#### Note: `std::process::Command` is Rust's equivalent of Go's `exec.Command`. For piped stdin/stdout, set `.stdin(Stdio::piped())` and `.stdout(Stdio::piped())` before spawning, then write to `child.stdin` and read from `child.wait_with_output()`. On Windows, `grep` is replaced with `findstr` and `ls` with `dir`; on Unix the original commands are used. Output will vary by system date and directory contents.
