Sample output; the date and time emitted will depend on when the example ran. Go's `log` and `log/slog` packages map to Rust's `log` + `env_logger` crates. `env_logger::Builder` provides the configurable format equivalent to Go's log flags. Structured JSON output mirrors Go's `log/slog` JSON handler.
___
##### Run Command:

`$ cargo run`

##### Results:

`2023/08/22 10:45:16 standard logger`
`2023/08/22 10:45:16 with micro`
`2023/08/22 10:45:16 with file/line`
`my:2023/08/22 10:45:16 from mylog`
`ohmy:2023/08/22 10:45:16 from mylog`
`from buflog:buf:2023/08/22 10:45:16 hello`
`{"time":"2023-08-22T10:45:16.904166391+00:00","level":"INFO","msg":"hi there"}`
`{"time":"2023-08-22T10:45:16.904178985+00:00","level":"INFO","msg":"hello again","key":"val","age":25}`
