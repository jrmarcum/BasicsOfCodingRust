// [_Command-line flags_](http://en.wikipedia.org/wiki/Command-line_interface#Command-line_option)
// are a common way to specify options for command-line programs.
// For example, in `wc -l` the `-l` is a command-line flag.
//
// Rust uses the `clap` crate with derive macros to handle flag parsing,
// similar to Go's `flag` package.

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// a string
    #[arg(long, default_value = "foo")]
    word: String,

    /// an int
    #[arg(long, default_value_t = 42)]
    numb: i32,

    /// a bool
    #[arg(long, default_value_t = false)]
    fork: bool,

    /// a string var
    #[arg(long, default_value = "bar")]
    svar: String,
}

fn main() {
    let args = Args::parse();

    println!("word: {}", args.word);
    println!("numb: {}", args.numb);
    println!("fork: {}", args.fork);
    println!("svar: {}", args.svar);

    // Collect trailing positional arguments (anything after `--`).
    let tail: Vec<String> = std::env::args()
        .skip(1)
        .skip_while(|a| a.starts_with('-'))
        .collect();
    println!("tail: {:?}", tail);
}
