##### Run Command:

`$ cargo run`

##### Results:

`2026-05-06T14:23:17+00:00`
`2012-11-01 22:08:41 UTC`
`2:23PM`
`Wed May  6 14:23:17 2026`
`2026-05-06T14:23:17.481532+00:00`
`0000-01-01 20:41:00 +0000 UTC`
`2026-05-06T14:23:17-00:00`
`parsing time "8:41PM" as "%a %b %e %H:%M:%S %Y": input contains invalid characters`
___
#### Note: Dates and times will vary with each run. Rust uses `chrono` for time formatting and parsing with strftime-style format strings, rather than Go's reference-time-based layout (`Mon Jan 2 15:04:05 MST 2006`). Go's `time.RFC3339` format maps to `chrono`'s `.to_rfc3339()`. The error message format for parse failures differs between Go and Rust.
