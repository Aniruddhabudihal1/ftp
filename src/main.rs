use crate::{client::client_call, server::server_implementation};
use std::io;

mod client;
mod server;

fn main() {
    println!("what do you want to run ?\nclient or server");
    let mut state = String::new();
    io::stdin().read_line(&mut state).unwrap();
    if state.to_lowercase().trim() == "server" {
        server_implementation();
    } else {
        client_call();
    }
}
