// Custom error types in Rust implement std::fmt::Display and std::error::Error.
// This mirrors Go's pattern of implementing the Error() method on a struct.
// Rust's error downcasting uses std::error::Error::downcast_ref, analogous
// to Go's errors.As.

use std::fmt;

// Custom error type with the "Error" suffix by convention.
#[derive(Debug)]
struct ArgError {
    arg: i32,
    message: String,
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.arg, self.message)
    }
}

impl std::error::Error for ArgError {}

fn f(arg: i32) -> Result<i32, Box<dyn std::error::Error>> {
    if arg == 42 {
        return Err(Box::new(ArgError {
            arg,
            message: "can't work with it".to_string(),
        }));
    }
    Ok(arg + 3)
}

fn main() {
    let err = f(42).unwrap_err();

    // Downcast the error to ArgError, analogous to Go's errors.As.
    if let Some(ae) = err.downcast_ref::<ArgError>() {
        println!("{}", ae.arg);
        println!("{}", ae.message);
    } else {
        println!("err doesn't match ArgError");
    }
}
