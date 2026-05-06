#### Rust does not have native variadic functions; a slice (`&[T]`) serves the same purpose. The output format is manually matched to Go's space-separated array display.
___
##### Run Command:

`$ cargo run`

##### Results:

`[1 2] 3`
`[1 2 3] 6`
`[1 2 3 4] 10`
