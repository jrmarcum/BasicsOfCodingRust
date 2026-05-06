// The `reqwest` crate provides excellent support for HTTP clients in Rust.
// In this example we'll use it to issue simple HTTP requests, mirroring
// Go's `net/http` package usage.

fn main() {
    // Issue an HTTP GET request to a server. `reqwest::blocking::get` is a
    // convenient shortcut that creates a client and sends a GET request.
    // The `blocking` module provides a synchronous API similar to Go's `http.Get`.
    let resp = reqwest::blocking::get("https://gobyexample.com")
        .expect("Failed to send request");

    // Print the HTTP response status.
    println!("Response status: {}", resp.status());

    // Print the first 5 lines of the response body.
    let body = resp.text().expect("Failed to read response body");
    for line in body.lines().take(5) {
        println!("{}", line);
    }
}
