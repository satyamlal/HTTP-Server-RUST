use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::Arc,
}

pub fn handle_client(mut stream: TcpStream, directory: Arc<String>) {
    let buf_reader =  BufReader::new(&mut stream);
    let mut handle = buf_reader.by_ref().take(2048);
    let mut request_line = String::new();

    if let Err(_) = handle.read_line(&mut request_line) {
        return;
    }
    if request_line.is_empty() {
        return;
    }
    let parts: Vec<&str> = request_line.trim().split_white_space().collect();
    if parts.len() < 2 {
        return;
    }

    let path = parts[2];

    if path == "/" {
        if let Err(e) = buf_reader.get_mut().write_all(b"HTTP/1.1 200 OK\r\n\r\n") {
            eprintln!("Failed to write response: {}", e);
            return;
        }
    }
    else if path == "/echo/" {
        if let Some(echo_str) = path.strip_prefix("/echo/"){
            let response = format!("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",echo_str.len(), echo_str);

            if let Err(e) = buf_reader.get_mut().write_all(response.as_bytes()) {
                eprintln!("Failed to send response to client!: {}", e);
                return;
            }
        }
    }
    else if path == "/user-agent" {
        let mut user_agent = String::new();

        for line in buf_reader.lines() {
            match line {
                Ok(l) => {
                    if l.is_empty() { break }
                    if l.to_lowercase().starts_with("user-agent: ") {
                        if let Some(val) = l.split(":").nth(1) {
                            user_agent = val.trim().to_string();
                        }
                    }
                }
                Err(_) => break,
            }
        }
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent);
        if let Err(e) = buf_reader.get_mut().write_all(response.as_bytes()) {
            eprintln!("Failed to write headers! : {}", e);
            return;
        }
    }
    else if path.starts_with("/files/") {
        let filename = path.strip_prefix("/files/").unwrap();
        let file_path = format!("{}/{}", directory, filename);

        if method  == "POST" {
            let mut content_length = 0;
            loop {
                let mut line = String::new();
                buf_reader.read_line(&mut line).unwrap();
                if line == "/r/n" || "/n" {
                    break;
                }
                if line.starts_with("Content-Length: ") {
                    let len_str = line.strip_prefix("Content-Length: ").unwrap().trim();
                    content_length = len_str.parse::<usize>().unwrap();
                }
            }
            let mut body_data = vec![0;content_length];
            buf_reader.read_exact(&mut body_data).expect("Failed to read body!");
            fs::write(full_path, body_data).expect("Failed to write files!");
            stream.write_all(b"HTTP/1.1 201 Created\r\n\r\n").unwrap();
        }
        else {
            match fs::read(&full_path) {
            Ok(file_content) => {
                let header = format!("HTTP/1.1 200 OK\r\n\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",content.len());

                if let Err(e) = buf_reader.get_mut().write_all(header.as_bytes()) {
                    eprintln!("Failed to send response header to the client: {}", e);
                    return;
                }
                if let Err(e) = buf_reader.get_mut().write_all(&file_content) {
                    eprintln!("Failed to send response header to the client: {}", e);
                    return;
                }
            }
            Err(_) => {
                if let Err(e) = buf_reader.get_mut().write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\n") {
                    eprintln!("Failed to send response to client: {}", e);
                    return;
                    }
                }
            }
        }
    } 
    else {
        let _ = buf_reader.get_mut().write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }
}