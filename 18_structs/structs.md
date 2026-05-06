##### Run Command:

`$ cargo run`

##### Results:

`{Bob 20}`
`{Alice 30}`
`{Fred 0}`
`&{Ann 40}`
`&{Jon 42}`
`Sean`
`50`
`51`
___
#### Rust uses `impl Display` to control how a struct prints, mirroring Go's default struct formatting (`{field value}`). Pointer fields in Go print with an `&` prefix; this is made explicit here.
