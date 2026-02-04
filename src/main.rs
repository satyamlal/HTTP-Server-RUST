use std::{
    thread,
    net::TcpListener,
    sync::Arc,
};

mod config;
mod handler;

use config::Config;
use handler::handle_client;

fn main() {

    let config = Config::build();
    let directory = Arc::new::(config.directory);

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind");
    println!("Server is listening on Port 4221 serving dir: {}", directory);

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