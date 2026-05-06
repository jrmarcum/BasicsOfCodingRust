#### Running the program shows that we pick up the value for `FOO` that we set in the program, but that `BAR` is empty.
___
##### Run Command:

`$ cargo run`

##### Results:

`FOO: 1`
`BAR: `
`...`
___
#### The total list of keys in the environment will depend on your particular machine.
___
#### If we set `BAR` in the environment first, the running program picks that value up.
___
##### Run Command:

`$ $env:BAR=2; cargo run` (for Windows PowerShell)

`$ BAR=2 cargo run` (for Unix-like environments)

##### Results:

`FOO: 1`
`BAR: 2`
`...`
