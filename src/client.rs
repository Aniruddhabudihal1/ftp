// This file contains code required for implementing a client

use core::panic;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn client_call() {
    let conn = TcpStream::connect("127.0.0.0:8080").unwrap();
    let mut buf_reader = BufReader::new(&conn);

    // Sends SYN message to client
    let con_ref = &mut conn.try_clone().unwrap();
    write_to_server(con_ref, String::from("hello\n"));

    // reading ACK message from the server
    //let ret = conn.read_exact(server_output_buffer.as_bytes());
    let server_output_buffer = read_from_server(&mut buf_reader);

    /*
    match ret {
        Ok(_) => println!("the read_line worked"),
        Err(er) => panic!(
            "the read_line did not work, and returned the following error : {}",
            er
        ),
    };
    */
    println!("test");
    println!("servers output : {}", server_output_buffer);

    if server_output_buffer == "hello client" {
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
    /*
        let ret = writer.write(file_name.as_bytes());

        match ret {
            Ok(_) => println!("successfully sent the file name to the server"),
            Err(er) => panic!(
                "The following error ocurred while sending file name to the string : {}",
                er
            ),
        };
    */

    let server_resp_for_file_sent = read_from_server(&mut buf_reader);
    if server_resp_for_file_sent == "true\n" {
        println!("the file exists and proceeding to ask the server to send the file");
    } else {
        panic!("the requested file does not exist");
    }

    /*
    let mut initial_file_response = String::new();
    let ret = con_ref.read(&mut initial_file_response.as_bytes());
    match ret {
        Ok(_) => {
            println!("successfully recieved a response from the server");
            if initial_file_response == "true\n" {
                println!("the file exists and proceeding to ask the server to send the file");
            } else {
                panic!(
                    "the file does not exist, later going to implement something where it goes back to asking the client to enter another file name"
                );
                //TODO
            }
        }
        Err(er) => panic!(
            "The following error ocurred while recieving the file status from the server : {}",
            er
        ),
    };
    */

    /*
        let cont = String::from("continue\n");
        let ret = writer.write(cont.as_bytes());
        match ret {
            Ok(_) => println!("successfully sent the file confirmation to the server"),
            Err(er) => panic!(
                "The following error ocurred while sending the file confirmation to the string : {}",
                er
            ),
        };
    */

    write_to_server(con_ref, String::from("continue\n"));

    let file_content = read_from_server(&mut buf_reader);

    /*
    let ret = con_ref.read_exact(&mut file_content.as_bytes());
    match ret {
        Ok(bytes_recieved) => println!("successfully recieved content from the server",),
        Err(er) => panic!(
            "something went wrong while trying to read from the server\nthe following is the errror recieved : {}",
            er
        ),
    };
    */

    let ret = File::create_new(file_name);
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
            println!("Write to the server commited successfully, where bytes were transferred")
        }
        Err(er) => println!(
            "Something went wrong while sending a message to the server\nerror : {}",
            er
        ),
    };
}

fn read_from_server(conn: &mut BufReader<&TcpStream>) -> String {
    // TODO
    let first_ret = conn.lines().next();
    match first_ret {
        Some(ret) => match ret {
            Ok(final_string) => final_string,
            Err(er) => panic!(
                "something went wrong while reading from the server\nerror returned : {}",
                er
            ),
        },
        None => String::new(),
    }
}
