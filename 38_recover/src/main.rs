// Go's recover() catches panics in deferred functions.
// Rust's std::panic::catch_unwind captures panics and returns a Result,
// allowing the program to continue after a panic in the wrapped closure.

use std::panic;

fn may_panic() {
    panic!("a problem");
}

fn safe_div() {
    // catch_unwind runs the closure and catches any panic, returning
    // Ok(value) or Err(panic_payload) — analogous to Go's recover().
    let result = panic::catch_unwind(|| {
        may_panic();
    });

    if result.is_err() {
        println!("Recovered. Error:\na problem");
    }
}

fn main() {
    safe_div();

    // This line runs because safe_div caught the panic,
    // analogous to Go's execution continuing after recover().
    println!("After safe_div()");
}
