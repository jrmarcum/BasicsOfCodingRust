#### Running our program shows a list sorted by string length, as desired.
___
##### Run Command:

`$ cargo run`

##### Results:

`[kiwi peach banana]`
___
#### In Rust, custom sorts use a closure passed directly to `sort_by`, rather than Go's approach of implementing a full interface (`Len`, `Less`, `Swap`).
