#### Maps are Rust's [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) (sometimes called _hashes_ or _dicts_ in other languages).
___
##### Run Command:

`$ cargo run`

##### Results:

`map: map[k1:7 k2:13]`
`v1:  7`
`len: 2`
`map: map[k1:7]`
`prs: false`
`map: map[bar:2 foo:1]`
___
#### Note: `HashMap` iteration order is non-deterministic; the helper function sorts keys for consistent output matching Go's `fmt.Println` map format.
