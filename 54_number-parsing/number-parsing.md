##### Run Command:

`$ cargo run`

##### Results:

`1.234`
`123`
`456`
`789`
`135`
`strconv.Atoi: parsing "wat": invalid digit found in string`
___
#### Note: Rust uses the `.parse()` method on string slices for number parsing, with the target type inferred or specified. Go's `strconv.Atoi` error message says `invalid syntax`; Rust's parse error says `invalid digit found in string`. For hex parsing, Rust requires stripping the `0x` prefix before calling `i64::from_str_radix(s, 16)` since Rust's `parse()` does not handle the `0x` prefix automatically.
