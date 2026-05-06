#### Rust iterators replace Go's `range`. Map iteration is non-deterministic; results are sorted here for consistent output. String iteration uses `.chars()`, giving Unicode scalar values printed as decimal integers like Go's rune values.
___
##### Run Command:

`$ cargo run`

##### Results:

`sum: 9`
`index: 1`
`a -> apple`
`b -> banana`
`key: a`
`key: b`
`0 103`
`1 111`
___
#### Note: Map iteration order is non-deterministic in Rust. The `a -> apple` / `b -> banana` and `key: a` / `key: b` pairs may appear in either order.
