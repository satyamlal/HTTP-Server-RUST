use std::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write, Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind");
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to establish a socket connection!");
            }
        }
    }
}

fn handle_client(mut stream:TcpStream){
    // feat: Buffering and Security to prevent DOS attacks
    let mut buf_reader = BufReader::new(&mut stream);
    let mut handle = buf_reader.by_ref().take(2048);
    let mut request_line = String::new();

    if let Err(e) = handle.read_line(&mut request_line) {
        println!("Failed to read line: {}", e);
        return;
    }

    // Checking for malicious cutoff or empty request
    if request_line.is_empty(){
        return;
    }

    let parts:Vec<&str> = request_line.trim().split_whitespace().collect();

    if parts.len() < 2 {
        println!("Invalid Request Format!");
        continue;
    }
    let path = parts[1];

    let response = if path == "/" {
        b"HTTP/1.1 200 OK\r\n\r\n";
    } else {
        b"HTTP/1.1 404 NOT FOUND\r\n\r\n";
    }
    
    if let Err(e) = buf_reader.get_mut().write_all(response) {
        println!("Failed to write response: {}", e);
    }
}
