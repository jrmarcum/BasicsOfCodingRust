// [Environment variables](http://en.wikipedia.org/wiki/Environment_variable)
// are a universal mechanism for conveying configuration information to programs.
// Let's look at how to set, get, and list environment variables.

use std::env;

fn main() {
    // To set a key/value pair, use `env::set_var`. To get a value for a key,
    // use `env::var`. This will return an `Err` if the key isn't present in
    // the environment; `unwrap_or_default()` gives an empty string in that case.
    env::set_var("FOO", "1");
    println!("FOO: {}", env::var("FOO").unwrap_or_default());
    println!("BAR: {}", env::var("BAR").unwrap_or_default());

    // Use `env::vars()` to list all key/value pairs in the environment.
    // This returns an iterator of `(String, String)` tuples.
    // Here we print all the keys.
    println!();
    for (key, _value) in env::vars() {
        println!("{}", key);
    }
}
