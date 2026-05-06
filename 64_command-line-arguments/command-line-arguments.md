#### To experiment with command-line arguments it's best to build a binary with `cargo build` first, or pass arguments after `--` with `cargo run`.
___
##### Run Command:

`$ cargo run -- a b c d`

##### Results:

`["...\\command-line-arguments.exe", "a", "b", "c", "d"]`
`["a", "b", "c", "d"]`
`c`

___
#### Note: The first element of `args` is the full path to the compiled binary (e.g. `target\debug\command-line-arguments.exe`), similar to Go's `os.Args[0]`. `args[3]` is index 3, which is `c` (index 0 = program path, 1 = `a`, 2 = `b`, 3 = `c`). This matches Go's `os.Args[3]` output of `c`. The program requires at least 4 arguments (indices 0–3) or it will panic.
