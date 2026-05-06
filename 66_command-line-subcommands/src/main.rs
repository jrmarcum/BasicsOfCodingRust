// Some command-line tools, like the `go` tool or `git`, have many
// *subcommands*, each with its own set of flags. For example, `go build`
// and `go get` are two different subcommands of the `go` tool.
//
// The `clap` crate lets us easily define subcommands that have their own flags.

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Foo subcommand
    Foo {
        #[arg(long, default_value_t = false)]
        enable: bool,
        #[arg(long, default_value = "")]
        name: String,
    },
    /// Bar subcommand
    Bar {
        #[arg(long, default_value_t = 0)]
        level: i32,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Foo { enable, name } => {
            println!("subcommand 'foo'");
            println!("  enable: {}", enable);
            println!("  name: {}", name);
        }
        Commands::Bar { level } => {
            println!("subcommand 'bar'");
            println!("  level: {}", level);
        }
    }
}
