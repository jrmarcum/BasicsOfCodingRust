##### Run Command:

`$ cargo run`

##### Results:

`2026-05-06 14:23:17.481532 UTC`
`2009-11-17 20:34:58.651387237 UTC`
`2009`
`November`
`17`
`20`
`34`
`58`
`651387237`
`UTC`
`Tue`
`true`
`false`
`false`
`143905h48m18.830144763s`
`143905.80523086557`
`8634348.384851934`
`518060903.0911161`
`518060903091116100`
`2026-05-06 14:23:17.481532 UTC`
`1993-06-01 02:46:40.821242474 UTC`
___
#### Note: The current time (first line) and all derived duration/add values will vary with each run. Rust uses `chrono` for time operations. Go's `time.Now()` becomes `chrono::Utc::now()`. Duration formatting differs: Go prints e.g. `25891h15m15.142266763s`; this Rust version formats similarly but the exact values depend on runtime. The weekday prints as the short form (e.g. `Tue`) from chrono's Debug format vs Go's full name (`Tuesday`).
