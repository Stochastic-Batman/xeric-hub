# xeric-hub

A minimalist, multi-threaded HTTP server

This project follows the final chapter of [*The Rust Programming Language*](https://doc.rust-lang.org/book/title-page.html) book to explore low-level networking, threads, and graceful shutdown.

## Features

* **TCP Listener:** Listens for incoming stream connections.
* **HTTP Parsing:** Basic request handling and response generation.
* **Thread Pool:** Manages connections using a pool of worker threads to prevent blocking.
* **Graceful Shutdown:** Cleans up resources and shuts down workers safely.

## Prerequisites

* [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) (cargo, rustc)

## Getting Started

1. **Clone the repository:**

```bash
git clone https://github.com/Stochastic-Batman/xeric-hub.git
cd xeric-hub
```

2. **Run the server:**

```bash
cargo run
```

3. **Access the hub:**

Open your browser and navigate to `http://127.0.0.1:7878`.

## Implementation Details

XericHub is built using:

* `std::net::TcpListener` for networking.
* `std::thread` and `std::sync` (Arc, Mutex, mpsc) for concurrency.
* A custom-built `ThreadPool` library.
