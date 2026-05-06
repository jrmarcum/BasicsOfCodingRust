##### Run Command:

`$ cargo run`

##### Results:

`2026-05-06 14:23:17.481532 UTC`
`1746541397`
`1746541397481`
`1746541397481532000`
`2026-05-06 14:23:17 UTC`
`2026-05-06 14:23:17.481532 UTC`
___
#### Note: Dates, times, and epoch values will vary with each run. Rust uses `std::time::SystemTime` for epoch arithmetic and `chrono` for formatting. Go's `time.Now().Unix()` becomes `SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()`; `UnixNano()` becomes `.as_nanos()`. Converting back from seconds/nanos uses `UNIX_EPOCH + Duration::from_secs(n)`.
