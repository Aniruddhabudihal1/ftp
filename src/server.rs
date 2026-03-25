// this file contains the code that is required by the server

use core::panic;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

pub fn server_implementation() {
    let conn = TcpListener::bind("127.0.0.0:8080").unwrap();
    let listen = conn.accept().unwrap();
    let mut stream = listen.0;

    let mut reader = BufReader::new(&stream);

    //let mut con_stream = TcpStream::connect("127.0.0.0:8080").unwrap();

    // recieving the SYN message from the client
    let mut buf = String::new();

    // this line works only when I control+c on the client
    let ret = reader.read_line(&mut buf);
    match ret {
        Ok(_) => println!("The read_line worked"),
        Err(er) => panic!(
            "the read_to_string did not work and gave the following errror : {}",
            er
        ),
    };

    println!("{}", buf);
    // responding with the ACK message to client
    if buf == "hello\n".to_string() {
        let _ = stream.write_all(String::from("hello client").as_bytes());
    } else {
        panic!("failed to recieve an appropriate SYN message from the client");
    }

    /*
    *
    let writing_con = TcpStream::connect("127.0.0.0:8080");
    match writing_con {
        Ok(peace) => println!("will be able to write to {:?}", peace),
        Err(er) => println!("Cannot write :( {}", er),
    }
    */
}
