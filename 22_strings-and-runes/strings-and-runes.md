Rust strings are UTF-8 byte slices. The `char` type represents a Unicode scalar value, equivalent to Go's `rune`. `s.len()` returns byte length; `s.chars().count()` returns the character count. `char_indices()` yields `(byte_offset, char)` pairs, equivalent to Go's `range` over a string. The `%#U` Go format maps to `U+{:04X} '{}'` in Rust.
___
##### Run Command:

`$ cargo run`

##### Results:

`Len: 18`
`e0 b8 aa e0 b8 a7 e0 b8 b1 e0 b8 aa e0 b8 94 e0 b8 b5 `
`Rune count: 6`
`U+0E2A 'ส' starts at 0`
`U+0E27 'ว' starts at 3`
`U+0E31 'ั' starts at 6`
`U+0E2A 'ส' starts at 9`
`U+0E14 'ด' starts at 12`
`U+0E35 'ี' starts at 15`
``
`Using DecodeRuneInString`
`U+0E2A 'ส' starts at 0`
`found so sua`
`U+0E27 'ว' starts at 3`
`U+0E31 'ั' starts at 6`
`U+0E2A 'ส' starts at 9`
`found so sua`
`U+0E14 'ด' starts at 12`
`U+0E35 'ี' starts at 15`
