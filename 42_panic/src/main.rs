// A `panic!` typically means something went unexpectedly
// wrong. Mostly we use it to fail fast on errors that
// shouldn't occur during normal operation, or that we
// aren't prepared to handle gracefully.

fn main() {

    // We'll use panic! to abort on an unexpected condition.
    // This is the only program in the series designed to panic.
    panic!("a problem");

    // A common use of panic! is to abort if a function
    // returns an error value that we don't know how to
    // (or want to) handle. For example:
    //
    // let _f = std::fs::File::create("/tmp/file")
    //     .expect("failed to create file");
}
