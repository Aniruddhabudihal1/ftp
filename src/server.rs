// this file contains the code that is required by the server

use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::{self, PathBuf},
    str::FromStr,
};

pub fn server_implementation() {
    let conn = TcpListener::bind("127.0.0.0:8080").unwrap();
    let mut stream = conn.accept().unwrap().0;

    let con_ref = &mut stream.try_clone().unwrap();
    let mut reader = BufReader::new(con_ref);
    //let mut writer = BufWriter::new(&stream);

    // recieving the SYN message from the client
    let mut buf = String::new();

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
    if buf == "hello\n" {
        //let _ = writer.write_all(String::from("hello client").as_bytes());
        write_to_client(&mut stream, String::from("hello client\n"));
    } else {
        panic!("failed to recieve an appropriate SYN message from the client");
    }

    // The client has sent what file to be transferred, and then based on availablity the file will
    // be transferred
    //let file_name = read_from_client(reader);
    let mut file_name = String::new();
    let ret = reader.read_line(&mut file_name);
    println!("File requested by the client is : {}", file_name);
    match ret {
        Ok(_) => println!("successfully read the clients desired file name"),
        Err(er) => panic!(
            "was not able to read the file name sent from the client and gave the following errror : {}",
            er
        ),
    };

    let file_path = path::PathBuf::from_str(&file_name).unwrap();
    let test = path::Path::is_file(&file_path.canonicalize().unwrap());
    println!("file exists status : {}", test);

    let ret = File::open(&file_name);
    match ret {
        Ok(_tmp) => {
            println!("The file exists on the server, and confirmation being sent to the client");
            let status = String::from("true\n");
            write_to_client(&mut stream, status);
            //let _ = writer.write_all(status.as_bytes());
        }
        Err(_) => {
            println!("The requested file does not exist on the server :( ");
            let status = String::from("false\n");
            write_to_client(&mut stream, status);
            //let _ = writer.write_all(status.as_bytes());
        }
    };

    let mut conf = String::new();
    let ret = reader.read_line(&mut conf);
    match ret {
        Ok(_) => println!("successfully read the clients confirmation for sending the file"),
        Err(er) => panic!(
            "was not able to read the file confirmation sent from the client and gave the following errror : {}",
            er
        ),
    };

    let mut file_content = String::new();
    let smt = File::open(&file_name);
    let mut f = match smt {
        Ok(fo) => {
            println!("was able to open the file");
            fo
        }
        Err(er) => panic!(
            "file path passed in is : {}\nfile probably does not exist : {}",
            file_name, er
        ),
    };
    let ret = f.read_to_string(&mut file_content);
    match ret {
        Ok(_) => {
            println!("successfully read the files contents and is ready to be sent to the client")
        }
        Err(er) => panic!(
            "Something went wrong while reading from the file, it gave the following errror : {}",
            er
        ),
    };

    write_to_client(&mut stream, file_content);

    /*
    let ret = writer.write(file_content.as_bytes());
    match ret {
        Ok(bytes_sent) => println!(
            "File content transfer complete, with {} bytes transmitted",
            bytes_sent
        ),
        Err(er) => panic!(
            "somethign went wrong while sending content to the client\nit gave the following error : {}",
            er
        ),
    };
    */
}

fn write_to_client(conn: &mut TcpStream, msg: String) {
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
