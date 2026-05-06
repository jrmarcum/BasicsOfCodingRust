##### Run Command:

`$ cargo run`

##### Results:

`{3 4}`
`12`
`14`
`{5}`
`78.53981633974483`
`31.41592653589793`
___
#### Rust uses `trait` instead of `interface`. The `measure` function accepts `&dyn Geometry` (a trait object). A `display` method on the trait provides the concrete-type string, mirroring Go's `fmt.Println(g)` which prints the underlying struct value.
