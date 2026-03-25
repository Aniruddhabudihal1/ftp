/*
use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::process::exit;
use std::{env, io, string};

// command line format : [client/server] ip_address port [filename]
fn main() {
    let args: Vec<String> = env::args().collect();
    let state = match args.get(2) {
        Some(bar) => {
            let lower_bar = bar.to_lowercase();
            if lower_bar == "server" {
                true
            } else if lower_bar == "client" {
                false
            } else {
                panic!("entered state to be in is wrong !\nchoose either a server or a client");
            }
        }
        _ => {
            panic!("please enter a valid state to be in, when passing the command line arguments");
        }
    };

    let ip_string: String = match args.get(3) {
        Some(ip) => ip.to_string(),
        _ => reenter(),
    };

    let ip_port = match args.get(4) {
        Some(port) => port.to_string(),
        _ => reenter(),
    };

    let connection_string = ip_string + ":" + &ip_port;
    println!(
        "The final connection of the client to the server is : {}",
        connection_string
    );

    let mut file_name = String::new();
    match args.get(5) {
        Some(bar) => file_name = bar.to_string(),
        _ => println!("I am guessing you are a server"),
    };

    if state {
        server_implementation(connection_string);
    } else {
        client_implementation(connection_string, file_name.to_string());
    }
}

fn reenter() -> String {
    println!("previously entered ip address or port was incorrect\nplease enter it again: ");
    let mut new_val = String::new();
    io::stdin().read_line(&mut new_val).unwrap();
    new_val
}

pub fn server_implementation(connection_string: String) {
    let conn = TcpListener::bind(connection_string);
    match conn {
        Ok(actual_conn) => {
            println!("connection with the client sucessfull : {:?}", actual_conn);
            for stream in actual_conn.incoming() {
                println!("Enters loop");
                handle_client(stream.unwrap());
            }
        }
        Err(ret_err) => {
            println!("connection failed ! \nError Found : {}", ret_err);
            exit(0);
        }
    }
}

fn handle_client(mut tcp_listener: TcpStream) {
    let mut reading_from_client = BufReader::new(&tcp_listener);
    let mut writing_to_client = BufWriter::new(&tcp_listener);

    // reading the SYN message from the client and then responding
    let mut resp = String::new();
    let ret = reading_from_client.read_to_string(&mut resp).unwrap();
    if ret != 0 {
        println!("failed to recieve the SYN message from the client");
    } else {
        println!("reponse from the client : {}", resp);
        println!("the latter");
    }

    // responding with the ACK message
    let ACK_from_server = String::from("hello from the server");
    writing_to_client.write_all(ACK_from_server.as_bytes());
    println!("confirmed connection with the client");

    // reading the filename from the client
    let mut file_name = String::new();
    let ret_value_from_client = reading_from_client.read_to_string(&mut file_name).unwrap();
    if ret_value_from_client == 0 {
        println!("was not able to read the filename being sent from the client");
        exit(1);
    }

    // searching the file in the server
    if let Ok(mut file_status) = File::open(file_name) {
        let mut file_content_in_string = String::new();
        file_status.read_to_string(&mut file_content_in_string);
        let _file_ret = writing_to_client.write_all(file_content_in_string.as_bytes());
    } else {
        println!("file does not exist !!");
        exit(1);
    }
}

use std::net::{self, TcpListener, TcpStream};
/*
 * firt data to be sent to the server is : a SYN message which needs to then get an ACK message from the server
 *
 * next send the filename (its whole path)
 *
 * should return the contents, with a checksum for the data being sent
 *
 * check the checksum sent, and then verify
 *
 * if not same, ask the server to send a file once again
 *
 * you can continue asking for other files using the same basic protocol
 *
 * once done you can send an end message
 **/

pub fn client_implementation(ip_to_connect_to: String, fileName: String) {
    let mut writing_from_client_to_server: BufWriter<TcpStream>;
    if let Ok(stream) = TcpStream::connect(&ip_to_connect_to) {
        writing_from_client_to_server = BufWriter::new(stream);
        println!("connection to the server from this client sucessful");
    } else {
        panic!(
            "connection to the server unsucessful, please check the ip to which you are connecting to once again"
        );
    }

    let mut reading_content_on_client_sent_from_server: BufReader<TcpStream>;
    if let Ok(stream) = TcpStream::connect(&ip_to_connect_to) {
        reading_content_on_client_sent_from_server = BufReader::new(stream);
    } else {
        panic!(
            "connection to the server unsucessful, please check the ip to which you are connecting to once again"
        );
    }

    // Task 1 : to send the SYN and then recieve the ACK message
    let bait = String::from("hello from the client");
    let ret = writing_from_client_to_server.write_all(bait.as_bytes());

    match ret {
        Ok(_) => println!("looks like it worked"),
        Err(er) => panic!("This is the error : {}", er),
    };

    let mut buf = String::new();
    let ret = reading_content_on_client_sent_from_server.read_to_string(&mut buf);

    if buf != "hello from the server" {
        panic!(
            "something went wrong, the ACK message recieved from the server is either wrong, or no message was recieved"
        );
    }

    // Task 2 : Send the path of the file that you want
    writing_from_client_to_server.write_all(fileName.as_bytes());
    let mut file_content = String::new();
    let _file_ret = reading_content_on_client_sent_from_server
        .read_to_string(&mut file_content)
        .unwrap();

    let foo1: Vec<&str> = fileName.rsplit("/").collect();
    let local_file_name = foo1.last().unwrap();

    let mut loacl_actual_file = File::create(local_file_name).unwrap();
    loacl_actual_file.write_all(file_content.as_bytes());

    println!("File transfer complete ! ");
}
*/

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
