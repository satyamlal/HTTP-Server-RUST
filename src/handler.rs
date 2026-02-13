use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    sync::Arc,
};

use crate::compress_data::compress_data;
use crate::parse_headers::parse_headers;

pub fn handle_client(mut stream: TcpStream, directory: Arc<String>) {
    loop {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut request_line = String::new();

        if let Err(_) = buf_reader.read_line(&mut request_line) { break; }
        if request_line.is_empty() { break; }

        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() < 2 { break; }

        let method = parts[0];
        let path = parts[1];
        let headers = parse_headers(&mut buf_reader);

        let connection_val = headers.get("connection").map(|s| s.as_str()).unwrap_or("keep-alive");
        let should_close = connection_val == "close";

        if path == "/" {
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
        } 
        else if path.starts_with("/echo/") {
            let echo_str = path.strip_prefix("/echo/").unwrap_or("");
            let supports_gzip = headers.get("accept-encoding")
                .map(|s| s.contains("gzip"))
                .unwrap_or(false);

            let body_bytes: Vec<u8>;
            let mut encoding_header = String::new();

            if supports_gzip {
                body_bytes = compress_data(echo_str);
                encoding_header = "Content-Encoding: gzip\r\n".to_string();
            } else {
                body_bytes = echo_str.as_bytes().to_vec();
            }

            let response = format!(
                "HTTP/1.1 200 OK\r\n{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
                encoding_header, body_bytes.len()
            );
            
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.write_all(&body_bytes);
        } 
        else if path == "/user-agent" {
            let agent = headers.get("user-agent").map(|s| s.as_str()).unwrap_or("unknown");
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                agent.len(), agent
            );
            let _ = stream.write_all(response.as_bytes());
        } 
        else if path.starts_with("/files/") {
            let filename = path.strip_prefix("/files/").unwrap();
            let file_path = format!("{}/{}", directory, filename);

            if method == "POST" {
                let content_length = headers.get("content-length")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0);

                let mut body_data = vec![0; content_length];
                if buf_reader.read_exact(&mut body_data).is_ok() {
                    if fs::write(&file_path, body_data).is_ok() {
                        let _ = stream.write_all(b"HTTP/1.1 201 Created\r\nContent-Length: 0\r\n\r\n");
                    } else {
                        let _ = stream.write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\n");
                    }
                }
            } else {
                match fs::read(&file_path) {
                    Ok(content) => {
                        let header = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                            content.len()
                        );
                        let _ = stream.write_all(header.as_bytes());
                        let _ = stream.write_all(&content);
                    }
                    Err(_) => {
                        let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
                    }
                }
            }
        } else {
            let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
        }
        if should_close { break; }
    }
}