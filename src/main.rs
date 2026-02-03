use std::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write, Read};
use std::thread;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind");
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
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
    let mut handle = buf_reader.by_ref().take(2048); // .take(2048) to prevent crashing via RAM OOM issue.
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

    // Reading header for /echo/ and /user-agent
    let response = if path == "/" {
        b"HTTP/1.1 200 OK\r\n\r\n".to_string();
    } else if path == "/echo/" {
        let echo_str = path.strip_prefix("/echo/").unwrap();
        let echo_len = echo_str.len();
        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n{}", echo_len, echo_str);
    } else if path == "/user-agent" {
        let mut user_agent_value = String::new();
        for line in buf_reader.lines(){
            break;
        }
        if line.starts_with("User-Agent: "){
            user_agent_value = line[12..].trim().to_string();
        }
        format!("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n{}", user_agent_value.len(), user_agent_value);
    } else {
        b"HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
    }
    
    if let Err(e) = buf_reader.get_mut().write_all(response.as_bytes()) {
        println!("Failed to write response: {}", e);
    }
}
