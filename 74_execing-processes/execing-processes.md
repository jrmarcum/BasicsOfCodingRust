#### When we run our program it runs `dir` (Windows) or `ls -a -l -h` (Unix) and exits with the same code.
___
##### Run Command:

`$ cargo run`

##### Results:

` Volume in drive D is Data`
` Volume Serial Number is XXXX-XXXX`
`...`
___
#### Note: Go's `syscall.Exec` is a Unix-only call that *replaces* the current process image (like the C `execve` syscall) — no code after it runs, and the original process disappears. On Unix, Rust can do the same with `std::os::unix::process::CommandExt::exec()`. On Windows there is no equivalent syscall, so this example uses `std::process::Command` to spawn a child process, waits for it to complete, and then exits with the child's exit code. The observable behavior is similar — the program runs the target command and exits — but on Windows the original process and the new process briefly coexist.
