Rust generics use trait bounds instead of Go's type constraints. Go's `comparable` maps to `PartialEq`; Go's `any` maps to an unconstrained type parameter. Generic types and functions work similarly in both languages. Note: Rust's `{:?}` debug format prints slices with commas (`[10, 13, 23]`) while Go prints without (`[10 13 23]`).
___
##### Run Command:

`$ cargo run`

##### Results:

`index of zoo: 2`
`list: [10, 13, 23]`
