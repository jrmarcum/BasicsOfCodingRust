#### Run the server with `cargo run`, then send a request to `/hello` and cancel it quickly with Ctrl+C to simulate context cancellation.
___
##### Run Command:

`$ cargo run`

`$ curl localhost:8090/hello`

##### Results:

`server: hello handler started`
`^C`
`server: context canceled`
`server: hello handler ended`
___
#### Note: Go uses `context.Context` (passed via `req.Context()`) for request-scoped cancellation. Rust does not have a built-in context type; this example uses a shared `Arc<AtomicBool>` cancellation flag polled in the handler. In async Rust (with `tokio`), cancellation is handled via `select!` on a `CancellationToken` from the `tokio-util` crate, which more closely mirrors Go's channel-based `ctx.Done()` pattern. The server waits up to 10 seconds — cancel the curl request early to see the cancellation message.
