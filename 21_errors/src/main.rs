// In Rust, errors are communicated via the Result<T, E> type rather than
// separate return values. This contrasts with Go's (value, error) tuple idiom.
// Rust's approach makes it impossible to ignore errors without explicit handling.

use std::fmt;

// f1 returns a simple string error, analogous to errors.New in Go.
fn f1(arg: i32) -> Result<i32, String> {
    if arg == 42 {
        return Err("can't work with 42".to_string());
    }
    Ok(arg + 3)
}

// Custom error type, analogous to Go's argError struct implementing the error interface.
#[derive(Debug)]
struct ArgError {
    arg: i32,
    prob: String,
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.arg, self.prob)
    }
}

impl std::error::Error for ArgError {}

fn f2(arg: i32) -> Result<i32, ArgError> {
    if arg == 42 {
        return Err(ArgError {
            arg,
            prob: "can't work with it".to_string(),
        });
    }
    Ok(arg + 3)
}

fn main() {
    // Test f1 with both a valid and an error-triggering argument.
    for i in [7, 42] {
        match f1(i) {
            Ok(r) => println!("f1 worked: {}", r),
            Err(e) => println!("f1 failed: {}", e),
        }
    }

    // Test f2 with both a valid and an error-triggering argument.
    for i in [7, 42] {
        match f2(i) {
            Ok(r) => println!("f2 worked: {}", r),
            Err(e) => println!("f2 failed: {}", e),
        }
    }

    // Access fields of the custom error type directly.
    if let Err(ae) = f2(42) {
        println!("{}", ae.arg);
        println!("{}", ae.prob);
    }
}
