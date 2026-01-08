# HTTP Server from Scratch (Rust)

A lightweight, multi-threaded HTTP server implementation built purely in Rust without using high-level web frameworks like Actix, Axum, or Rocket.

This project focuses on understanding the low-level details of the HTTP/1.1 protocol, TCP handling, and concurrent connection management.

## 🚀 Features implemented

- **TCP Connection Handling**: Binds to a port and listens for incoming TCP streams.
- **Request Parsing**: 
  - Extracts URL paths and HTTP methods.
  - Reads and parses HTTP headers.
  - Reads request bodies.
- **Response Generation**: 
  - Constructs valid HTTP/1.1 status lines and headers.
  - Sends text and file bodies.
- **File Server**: capable of serving static files from a directory.
- **Concurrency**: Handles multiple client connections simultaneously using a thread pool/threading.

## 🛠 Progress Tracker

- [x] Extract URL path
- [x] Respond with body
- [x] Read header
- [x] Concurrent connections
- [x] Return a file
- [x] Read request body
- [x] HTTP Compression (Gzip)
- [x] Persistent Connections (Keep-Alive)

## 💻 How to Run

1. Ensure you have Rust installed.
2. Clone the repository.
3. Run the server:
   ```bash
   # Defaults to localhost:4221 (or your configured port)
   cargo run

4. Test with curl:
curl -v [http://127.0.0.1:4221/](http://127.0.0.1:4221/)

## 🧩 Technical Stack
- **Language:** Rust
- **Networking**: std::net (TcpListener, TcpStream)
- **Threading:** std::thread
- **File I/O:** std::fs


