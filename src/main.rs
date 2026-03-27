use crate::{client::client_call, server::server_implementation};
use std::env;

mod client;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(2) {
        Some(_) => client_call(),
        _ => server_implementation(),
    }
}
