# HTTP Server from Scratch (Rust)

A lightweight, multi-threaded HTTP server implementation built purely in Rust.

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
```bash
curl -v [http://127.0.0.1:4221/](http://127.0.0.1:4221/)
```

## 🧩 Technical Stack & Architecture
- **Language:** Rust (2021 Edition)
- **Architecture:** Multi-threaded, Blocking I/O

### Core Modules (std):
- **std::net:** Raw TCP socket management (TcpListener, TcpStream).
- **std::io:** Buffered reading/writing (BufReader) for efficient protocol parsing.
- **std::sync:** Atomic Reference Counting (Arc) for safe state sharing across threads.
- **std::thread:** Spawning threads for concurrent connection handling.

### External Dependencies:
- **flate2:** Implements the Gzip compression algorithm (DEFLATE).
