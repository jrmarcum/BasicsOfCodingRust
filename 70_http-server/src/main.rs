// Writing a basic HTTP server using Rust's standard library `std::net::TcpListener`.
// This mirrors Go's `net/http` server example using only stdlib — no external crates.
// For production Rust HTTP servers, use frameworks like `axum` or `actix-web`.

use std::io::{Read, Write};
use std::net::TcpListener;

/// Handle a request path and return a full HTTP/1.1 response string.
fn hello_handler(path: &str) -> String {
    match path {
        "/hello" => {
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 6\r\n\r\nhello\n"
                .to_string()
        }
        "/headers" => {
            // In a real server we'd echo the request headers here.
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n(headers echoed here)\n"
                .to_string()
        }
        _ => "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\n404 not found\n"
            .to_string(),
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:8090").unwrap();
    println!("Listening on localhost:8090");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap_or(0);

        // Parse the request line to extract the path.
        let request = String::from_utf8_lossy(&buf[..n]);
        let path = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        let response = hello_handler(path);
        stream.write_all(response.as_bytes()).unwrap();
    }
}
