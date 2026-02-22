use std::{
    thread,
    net::TcpListener,
    sync::Arc,
};

mod config;
mod handler;
mod compress_data;
mod parse_headers;

use config::Config;
use handler::handle_client;

fn main() {
    let config = Config::build();
    let directory = Arc::new(config.directory);

    let listener = match TcpListener::bind("127.0.0.1:4221") {
        Ok(l) => l,
        Err(e) => {
            println!("CRITICAL: Failed to bind. OS Error: {}", e);
            process::exit(1);
        }
    }
    println!("Server is listening on Port 4221 serving dir: {}", directory);

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                // cloning the pointer for cheap & fast processing rather than cloning the data
                let dir_copy = Arc::clone(&directory); 
                thread::spawn(move || handle_client(stream, dir_copy)); // concurrent connections: Multi-threading
            }
            Err(_) => {
                eprintln!("Failed to establish a socket connection!");
            }
        }
    }
}