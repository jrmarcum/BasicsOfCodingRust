#### Running our URL parsing program shows all the different pieces that we extracted.
___
##### Run Command:

`$ cargo run`

##### Results:

`postgres`
`user:pass`
`user`
`pass`
`host.com`
`host.com`
`5432`
`/path`
`f`
`k=v`
`map[k:[v]]`
`v`
___
#### Note: Rust uses the `url` crate for URL parsing. Go's `url.Parse(s)` becomes `Url::parse(s)`. Go's `u.Host` returns `"host.com:5432"` (host+port together); the `url` crate separates them via `.host_str()` and `.port()`, so both are printed separately to match Go's `net.SplitHostPort` output. Query params are accessed via `.query_pairs()` instead of `url.ParseQuery`.
