// Rust has first-class enum types, unlike Go which uses iota constants.
// The Display trait replaces Go's fmt.Stringer interface.

use std::fmt;

#[derive(Debug, PartialEq)]
enum ServerState {
    Idle,
    Connected,
    Error,
    Retrying,
}

impl fmt::Display for ServerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ServerState::Idle => "idle",
            ServerState::Connected => "connected",
            ServerState::Error => "error",
            ServerState::Retrying => "retrying",
        };
        write!(f, "{}", name)
    }
}

// transition emulates a state transition for a server.
fn transition(s: ServerState) -> ServerState {
    match s {
        ServerState::Idle => ServerState::Connected,
        ServerState::Connected | ServerState::Retrying => ServerState::Idle,
        ServerState::Error => ServerState::Error,
    }
}

fn main() {
    let ns = transition(ServerState::Idle);
    println!("{}", ns);

    let ns2 = transition(ns);
    println!("{}", ns2);
}
