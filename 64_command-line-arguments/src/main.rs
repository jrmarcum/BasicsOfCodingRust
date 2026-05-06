// [_Command-line arguments_](http://en.wikipedia.org/wiki/Command-line_interface#Arguments)
// are a common way to parameterize execution of programs.
// For example, `cargo run -- a b c` passes `a`, `b`, and `c`
// as arguments to the program.

use std::env;

fn main() {
    // `env::args()` provides access to raw command-line arguments.
    // Note that the first value in this collection is the path to the
    // program, and `&args[1..]` holds the arguments to the program.
    let args: Vec<String> = env::args().collect();
    let args_without_prog = &args[1..];

    // You can get individual args with normal indexing.
    let arg = &args[3];

    println!("{:?}", args);
    println!("{:?}", args_without_prog);
    println!("{}", arg);
}
