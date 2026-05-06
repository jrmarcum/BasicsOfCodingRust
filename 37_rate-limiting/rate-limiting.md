Running our program we see the first batch of requests handled once every ~200ms. For the second batch, the first 3 are served immediately due to the burst allowance, then the remaining 2 are served with ~200ms delays each. Timestamps reflect actual run time. Go's ticker-based limiter is replaced by `thread::sleep` for basic limiting and a pre-filled `sync_channel` for burst limiting.
___
##### Run Command:

`$ cargo run`

##### Results:

`request 1 1729295898.687438000`
`request 2 1729295898.887471000`
`request 3 1729295899.087238000`
`request 4 1729295899.287338000`
`request 5 1729295899.487331000`
`request 1 1729295899.487578000`
`request 2 1729295899.487645000`
`request 3 1729295899.487676000`
`request 4 1729295899.687483000`
`request 5 1729295899.887542000`
