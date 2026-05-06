##### Run Command:

`$ cargo run -- foo --enable --name=joe`

##### Results:

`subcommand 'foo'`
`  enable: true`
`  name: joe`
___
#### Now try bar.
___
##### Run Command:

`$ cargo run -- bar --level 8`

##### Results:

`subcommand 'bar'`
`  level: 8`
___
#### But bar won't accept foo's flags.
___
##### Run Command:

`$ cargo run -- bar --enable`

##### Results:

`error: unexpected argument '--enable' found`
___
#### Note: `clap` subcommands use `--` (double-dash) long flags. Trailing positional arguments after flags are not shown here as `clap` does not expose them by default — add `#[arg(trailing_var_arg = true)]` to capture them if needed. The Go example uses `flag.Args()` to show trailing args; the behavior is similar in concept.
