use std::{TcpListener, TcpStream};
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
    let mut buf = [0;512];
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read the incoming connection!");
        if bytes_read == 0 {
            return;
        }
        stream.write_all(&buf[0..bytes_read]).expect("Failed to write to client");
    }
}
