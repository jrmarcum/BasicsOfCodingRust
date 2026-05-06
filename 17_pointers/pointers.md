#### `zeroval` doesn't change `i` in `main`, but `zeroptr` does because it holds a mutable reference. Rust enforces this at compile time through the borrow checker.
___
##### Run Command:

`$ cargo run`

##### Results:

`initial: 1`
`zeroval: 1`
`zeroptr: 0`
`pointer: 0xc0000b4008`
___
#### Note: The pointer address on the last line changes each time the program is run.
