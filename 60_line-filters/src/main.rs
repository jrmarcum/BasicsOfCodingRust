// A line filter is a common type of program that reads
// input on stdin, processes it, and then prints some
// derived result to stdout. `grep` and `sed` are common
// line filters.

// Here's an example line filter in Rust that writes a
// capitalized version of all input text.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Lock stdin for efficient line-by-line reading.
    // BufRead::lines() is the equivalent of bufio.Scanner.
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => println!("{}", l.to_uppercase()),
            Err(e) => {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
