// This file contains code required for implementing a client

use core::panic;
use std::io::{self, Read};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn client_call() {
    let mut conn = TcpStream::connect("127.0.0.0:8080").unwrap();

    // Sends SYN message to client
    let con_ref = &mut conn.try_clone().unwrap();
    write_to_server(con_ref, String::from("hello\n"));

    // reading ACK message from the server
    let server_output_buffer = read_from_server(&mut conn);

    println!("test");
    println!("servers output : {}", server_output_buffer);

    if server_output_buffer == "hello client\n" {
        println!("Connection with the server confirmed with the ACK message");
    } else {
        panic!("connection with the server failed");
    }

    // reading file name from user input and then sending it to the server
    let mut file_name = String::new();
    println!("Enter file to be searched : ");
    io::stdin()
        .read_line(&mut file_name)
        .expect("was not able to read file name for some reason");

    write_to_server(con_ref, file_name.clone());

    let server_resp_for_file_sent = read_from_server(&mut conn);
    if server_resp_for_file_sent == "true\n" {
        println!("the file exists and proceeding to ask the server to send the file");
    } else {
        panic!("the requested file does not exist");
    }

    write_to_server(con_ref, String::from("continue\n"));

    let file_content = read_file_content(&mut conn);

    println!("the file content recieved is : {}", file_content);

    println!("enter the name of the output file : ");
    let mut output_file_name = String::new();
    io::stdin().read_line(&mut output_file_name).unwrap();
    output_file_name.pop();

    let ret = File::create_new(output_file_name);
    match ret {
        Ok(mut f) => {
            let _ = f.write_all(file_content.as_bytes());
            println!("successfully created the file !! ");
        }
        Err(er) => panic!(
            "check if the file already exists in the current directory, got error : {}",
            er
        ),
    };
}

fn write_to_server(conn: &mut TcpStream, msg: String) {
    let ret = conn.write_all(msg.as_bytes());
    match ret {
        Ok(_) => {
            println!("wrote to the server successfully")
        }
        Err(er) => println!(
            "Something went wrong while sending a message to the server\nerror : {}",
            er
        ),
    };
}

fn read_file_content(conn: &mut TcpStream) -> String {
    let mut file_content = String::new();
    let real_con = conn.try_clone().unwrap();
    let mut reader = BufReader::new(real_con);
    let _ = reader.read_to_string(&mut file_content);
    file_content
}

fn read_from_server(conn: &mut TcpStream) -> String {
    let mut resp = String::new();
    let real_con = conn.try_clone().unwrap();
    let mut reader = BufReader::new(real_con);
    let _ = reader.read_line(&mut resp);
    resp
}
