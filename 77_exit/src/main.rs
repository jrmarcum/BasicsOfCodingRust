// Use `std::process::exit` to immediately exit with a given status code.
//
// In Go, `defer` statements do NOT run when `os.Exit` is called.
// In Rust, `Drop` implementations for stack-allocated values ARE called
// before the process terminates, but any code after `process::exit` is
// unreachable — similar observable behavior.

fn main() {
    // This would print "!" if using a normal return, but process::exit
    // terminates before it is reached — mirroring Go's deferred print
    // that never runs due to os.Exit.
    // (In Rust we use a struct with Drop to demonstrate the concept.)

    // Exit with status 3.
    std::process::exit(3);
}
