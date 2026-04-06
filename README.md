# 🦀 Rust Multithreaded Web Server

A blazingly fast, concurrent HTTP web server built entirely from scratch using the Rust Standard Library.

This project demonstrates low-level systems programming concepts including custom thread pools, channel-based message passing, dynamic file serving, and thread-safe shared state management using atomic reference counting and mutexes.

## ✨ Features

- **Custom Thread Pool Architecture**: Manages a fixed number of worker threads to handle concurrent incoming HTTP connections efficiently without overwhelming system resources.
- **Dynamic File Serving**: Parses raw HTTP requests to dynamically serve HTML, CSS, JavaScript, and Image files from the `public` directory.
- **Automatic MIME Type Inference**: Inspects file extensions to correctly label and serve binary and text data via HTTP headers.
- **Thread-Safe Visitor Counter**: A live site counter built with `Arc<Mutex<usize>>`, guaranteeing zero data races across concurrent connections, injected dynamically via Server-Side Rendering (SSR).
- **Concurrency & Message Passing**: Uses `mpsc` (Multiple Producer, Single Consumer) channels to safely dispatch connection tasks from the main TCP listener to available worker threads.
- **Real-Time Terminal Logging**: Actively logs connection IP addresses and request paths for active monitoring.
- **Memory Safe**: Leverages Rust's ownership model and borrow checker to guarantee memory safety and thread safety at compile time.

## 🚀 Getting Started

### Prerequisites
Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Running Locally

1. Clone this repository:
```bash
    git clone https://github.com/revrio/Rust-web-server
    cd Rust-web-server
```

2. Start the server (Release mode recommended for performance):
```bash
    cargo run --release
```
3. Open your web browser and navigate to:

    http://localhost:8000

## 📂 Project Structure
```bash
Rust-web-server/
├── Cargo.toml          # Project metadata and dependencies
├── src/
│   ├── main.rs         # TCP listener, HTTP routing, MIME types, and state logic
│   └── lib.rs          # Custom ThreadPool, Worker, and Job implementation
└── public/             # Web content directory
    ├── index.html      # Server dashboard
    ├── style.css       # Custom styling
    ├── script.js       # Client-side logic
    └── 404.html        # Custom Not Found page
    
```
#### Adding Content
Place your HTML, CSS, JavaScript and other static files in the public directory. The server will automatically serve them when requested based on the URL path.

#### Visitor Counter
The server includes a built-in, thread-safe visitor counter. It is automatically injected into the index.html page via Server-Side Rendering (SSR) every time the root route (/) is requested.

## 🏗️ Architecture Under the Hood

Unlike servers that spawn a new thread for every single connection (which can lead to resource exhaustion), this server utilizes a highly optimized **Worker Pool**:

1. **The Listener:** The main thread listens for TCP connections on port `8000`.
2. **The Envelope:** The connection stream and routing instructions are wrapped in a closure (`Box<dyn FnOnce() + Send + 'static>`).
3. **The Channel:** The closure is sent down an `mpsc` channel to the worker pool.
4. **The Workers:** 4 idle worker threads continuously wait on a shared `Arc<Mutex<Receiver>>`. The first available worker locks the receiver, grabs the job, releases the lock immediately, and executes the HTTP response logic.

This guarantees high throughput, low latency, and absolute thread safety.