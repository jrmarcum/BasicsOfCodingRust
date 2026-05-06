##### Run Command:

`$ cargo run`

##### Results:

`{1 2}`
`{x:1 y:2}`
`main.Point{x:1, y:2}`
`Point`
`true`
`123`
`1110`
`!`
`1c8`
`78.900000`
`1.234000e+08`
`1.234000E+08`
`"string"`
`"\"string\""`
`6865782074686973`
`0x... (pointer address varies)`
`|    12|   345|`
`|  1.20|  3.45|`
`|1.20  |3.45  |`
`|   foo|     b|`
`|foo   |b     |`
`a string`
`an error`
___
#### Note: `an error` is written to stderr. The pointer address changes each run. Rust uses `Point` (PascalCase) where Go uses `point` (lowercase); `main.Point` mirrors Go's `main.point` package-qualified name.
