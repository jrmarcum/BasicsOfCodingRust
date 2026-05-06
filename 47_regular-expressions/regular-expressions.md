##### Run Command:

`$ cargo run`

##### Results:

`true`
`true`
`peach`
`[0 5]`
`["peach", "ea"]`
`[0, 5, 1, 3]`
`["peach", "punch", "pinch"]`
`[[0, 5, 1, 3], [6, 11, 7, 9], [12, 17, 13, 15]]`
`["peach", "punch"]`
`true`
`p([a-z]+)ch`
`a <fruit>`
`a PEACH`
___
#### Note: Rust's `regex` crate API differs slightly from Go's `regexp` package. Go's `r.Match([]byte)` becomes `r.is_match(str)` in Rust (the crate works on `&str` directly). The submatch output uses Rust's debug format with commas. Go prints the compiled regex as `p([a-z]+)ch`; Rust's `r.as_str()` returns the same pattern string.
