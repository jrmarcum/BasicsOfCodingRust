#### Run the server with `cargo run`, then access the `/hello` route with curl or a browser. The server runs until interrupted with Ctrl+C.
___
##### Run Command:

`$ cargo run`

`$ curl localhost:8090/hello`

##### Results:

`hello`
___
#### Note: This is a minimal stdlib implementation using `std::net::TcpListener` with manual HTTP/1.1 response formatting. Go's `net/http` package provides a full HTTP framework; for production Rust HTTP servers use `axum` or `actix-web`. The `/headers` route is stubbed — a production implementation would echo the parsed request headers. The server handles one connection at a time (no threading); use `thread::spawn` or `tokio` for concurrent connections.
