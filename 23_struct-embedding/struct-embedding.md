Rust does not support struct embedding. Composition is used instead: the inner type becomes a named field (`base: Base`). Methods of the inner type are not automatically promoted — they must be explicitly delegated. Where Go allows `co.num` directly via embedding, Rust requires `co.base.num`. Traits replace Go's interfaces; a struct implements a trait explicitly rather than implicitly.
___
##### Run Command:

`$ cargo run`

##### Results:

`co={num: 1, str: some name}`
`also num: 1`
`describe: base with num=1`
`describer: base with num=1`
