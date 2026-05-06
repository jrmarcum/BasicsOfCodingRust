#### To experiment with command-line flags, pass arguments after `--` when using `cargo run`.
___
##### Run Command:

`$ cargo run -- --word=opt --numb=7 --fork --svar=flag`

##### Results:

`word: opt`
`numb: 7`
`fork: true`
`svar: flag`
`tail: []`
___
#### Note that if you omit flags they automatically take their default values.
___
##### Run Command:

`$ cargo run -- --word=opt`

##### Results:

`word: opt`
`numb: 42`
`fork: false`
`svar: bar`
`tail: []`
___
#### Use `--help` to get automatically generated help text for the command-line program.
___
##### Run Command:

`$ cargo run -- --help`

##### Results:

`Usage: command-line-flags.exe [OPTIONS]`
``
`Options:`
`      --word <WORD>  a string [default: foo]`
`      --numb <NUMB>  an int [default: 42]`
`      --fork         a bool`
`      --svar <SVAR>  a string var [default: bar]`
`  -h, --help         Print help`
___
#### Note: `clap` uses `--` (double-dash) long flags by default, whereas Go's `flag` package uses single-dash `-`. The `clap` crate provides richer help output formatting than Go's `flag` package.
