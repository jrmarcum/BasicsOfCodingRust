##### Run Command:

`$ cargo run`

##### Results:

`Contains:  true`
`Count:     2`
`HasPrefix: true`
`HasSuffix: true`
`Index:     1`
`Join:      a-b`
`Repeat:    aaaaa`
`Replace:   f00`
`Replace:   f0o`
`Split:     [a b c d e]`
`ToLower:   test`
`ToUpper:   TEST`
`Len: 5`
`Char: 101`
___
#### Note: In Rust, string functions are methods on `&str`/`String` rather than free functions in a separate package. All output is identical to Go.
