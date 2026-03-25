// this file contains the code that is required by the server

use std::{
    fmt::write,
    io::{BufReader, Read},
    net::{TcpListener, TcpStream},
};

pub fn server_implementation() {
    let conn = TcpListener::bind("127.0.0.0:8080").unwrap();
    let listen = conn.accept().unwrap();
    let mut stream = listen.0;

    let mut buf = String::new();
    stream.read_to_string(&mut buf);

    println!("buffer sent from client : {}", buf);

    /*
    *
    let writing_con = TcpStream::connect("127.0.0.0:8080");
    match writing_con {
        Ok(peace) => println!("will be able to write to {:?}", peace),
        Err(er) => println!("Cannot write :( {}", er),
    }
    */
}
