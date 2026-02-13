use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub fn parse_headers(buf_reader: &mut BufReader<&mut TcpStream>) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line == "\r\n" || line == "\n" {
                    break;
                }
                if let Some((key, value)) = line.split_once(":") {
                    headers.insert(key.trim().to_lowercase(), value.trim().to_string());
                }
            }
            Err(_) => break,
        }
    }
    headers
}