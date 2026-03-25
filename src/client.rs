// This file contains code required for implementing a client

use std::{
    io::{BufWriter, Write},
    net::{TcpListener, TcpStream},
};

pub fn client_call() {
    let msg = String::from("hello");

    let mut con = TcpStream::connect("127.0.0.0:8080").unwrap();
    let mut stream = con.write_all(msg.as_bytes());
}
