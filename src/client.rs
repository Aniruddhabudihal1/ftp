// This file contains code required for implementing a client

use core::panic;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

pub fn client_call() {
    let msg = String::from("hello\n");

    let mut con = TcpStream::connect("127.0.0.0:8080").unwrap();

    //let mut con_read = TcpListener::bind("127.0.0.0:8080").unwrap();
    //let tmp = con_read.accept().unwrap();
    //let mut stream = tmp.0;

    // Sends SYN message to client
    let ret = con.write(msg.as_bytes());

    match ret {
        Ok(_) => println!("the write worked"),
        Err(er) => panic!(
            "the write did not work, and returned the following error : {}",
            er
        ),
    }

    let mut server_output_buffer = String::new();

    // reading ACK message from the server
    let mut reader = BufReader::new(con);
    let ret = reader.read_line(&mut server_output_buffer);
    match ret {
        Ok(_) => println!("the read_line worked"),
        Err(er) => panic!(
            "the read_line did not work, and returned the following error : {}",
            er
        ),
    };

    println!("{}", server_output_buffer);

    if server_output_buffer == "hello client" {
        println!("Connection with the server confirmed");
    } else {
        panic!("connection with the server failed");
    }

    /*
        let _ = con.read_to_string(&mut server_output_buffer);
        println!(
            "probably this will just be empty : {}",
            server_output_buffer
        );
        if server_output_buffer == "hello client" {
            println!("Connection with the server confirmed");
        } else {
            panic!("connection with the server failed");
        }
    */
}
