use std::{
    env, fs, thread,
    net::{TcpListener, TcpStream},
    io::{BufReader, BufRead, Write, Read},
    sync::Arc,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind");
    println!("Server is listening on Port 4221");

    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 2 args[1] == "--directory" {
        args[2].clone()
    } else {
        String::from("."); // current directory - Default
    }
    let directory = Arc::new(dir);

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                // cloning the pointer for cheap & fast processing rather than cloning the data
                let dir_copy = Arc::clone(&directory); 
                thread::spawn(move || handle_client(stream, dir_copy)); // concurrent connections: Multi-threading
            }
            Err(e) => {
                eprintln!("Failed to establish a socket connection!");
            }
        }
    }
}

fn handle_client(mut stream:TcpStream, directory: Arc<String>){
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
        return;
    }
    let path = parts[1];

    // Reading header for /echo/ and /user-agent
    if path == "/" {
        let response = b"HTTP/1.1 200 OK\r\n\r\n";
        let _ = buf_reader.get_mut().write_all(response.as_bytes());
    }
    else if path == "/echo/" {
        let echo_str = path.strip_prefix("/echo/").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n{}", echo_str.len(), echo_str);
    }
    else if path == "/user-agent" {
        let mut user_agent_value = String::new();

        for line in buf_reader.lines(){
            match line {
                Ok(l) => {
                    if l.is_empty() {
                        break;
                    }
                    if l.to_lowercase().starts_with("user-agent:") {
                        if let Some(val) = l.split(':').nth(1) {
                            user_agent_value = val.trim().to_string();
                        }
                    }
                }
                Err(_) => break,
            }
        }

        let response = format!("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n{}", user_agent_value.len(), user_agent_value);
        let _ = buf_readers.get_mut().write_all(response.as_bytes());

    } else if path.starts_with("/files/") {
        let filename = path.strip_prefix("/files/").unwrap();
        let full_path = format!("{}/{}", directory, filename); // /tmp/ + foo = /tmp/foo

        match fs::read(full_path){
            Ok(file_content) => {
                let header = format!("HTTP/1.1 200 Ok\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n", file_content.len());
            }
            // send headers (string)
            buf_reader.get_mut().write_all(response.as_bytes());
            // send file contents (Bytes)
            buf_reader.get_mut().write_all(&file_content);
            Err(_) => {
                let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                let _ = buf_reader.get_mut().write_all(response.as_bytes());
            }
        }
    } else {
        let response = b"HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
        let _ = buf_reader.get_mut().write_all(response.as_bytes());
    }
}
