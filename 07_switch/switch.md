#### Rust uses `match` instead of `switch`. Type inspection uses a generic function with `std::any::TypeId` and `type_name`, mirroring Go's type switch. In Rust the string type is `&str` rather than Go's `string`.
___
##### Run Command:

`$ cargo run`

##### Results:

`Write 2 as two`
`It's a weekday`
`It's after noon`
`I'm a bool`
`I'm an i32`
`Don't know type &str`
___
#### Note: The weekday line (`It's a weekday` or `It's the weekend`) and the time-of-day line (`It's before noon` or `It's after noon`) will reflect the actual day and time when the program is run.
