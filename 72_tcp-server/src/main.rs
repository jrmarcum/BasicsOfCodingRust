// The `std::net` module provides the tools we need to easily build
// TCP socket servers, mirroring Go's `net` package.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // `TcpListener::bind` starts the server on the given address.
    let listener = TcpListener::bind("0.0.0.0:8090").unwrap();
    println!("Listening on :8090");

    // Loop indefinitely to accept new client connections.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a thread to handle each connection so the main loop
                // can continue accepting more connections.
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

/// Handle a single client connection: read one line, respond with ACK in uppercase.
fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;

    let mut message = String::new();
    match reader.read_line(&mut message) {
        Ok(0) => return, // Connection closed.
        Ok(_) => {}
        Err(e) => {
            eprintln!("Read error: {}", e);
            return;
        }
    }

    // Create and send a response: ACK the message in uppercase.
    let trimmed = message.trim_end_matches(|c| c == '\n' || c == '\r');
    let response = format!("ACK: {}\n", trimmed.to_uppercase());
    if let Err(e) = writer.write_all(response.as_bytes()) {
        eprintln!("Server write error: {}", e);
    }
}
