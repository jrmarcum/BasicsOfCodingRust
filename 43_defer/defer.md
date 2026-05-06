#### Running the program confirms that the cleanup runs after writing, matching Go's defer behavior.
___
##### Run Command:

`$ cargo run`

##### Results:

`creating`
`writing`
`closing`
___
#### Rust does not have a `defer` keyword. Instead, Rust uses RAII (Resource Acquisition Is Initialization): the `Drop` trait runs cleanup automatically when a value goes out of scope. This provides the same deterministic cleanup guarantee as Go's `defer`, but is built into the ownership system rather than being a language keyword. A `DeferGuard` wrapper struct can be used to emulate Go's defer pattern with an arbitrary closure.
