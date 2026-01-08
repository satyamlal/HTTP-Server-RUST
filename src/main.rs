use std::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Could not bind");
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                let _stream = stream.unwrap();
                eprintln!("Connection established!");
            }
            Err(e) => {
                eprintln!("Failed to establish a socket connection!");
            }
        }
    }
}
