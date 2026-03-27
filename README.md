# http-server

A lightweight, async HTTP server library built in Rust on top of [Tokio](https://tokio.rs/). Designed as a learning project to explore async I/O, TCP streams, HTTP parsing, and trait-based response systems.

> **Note:** This library has not been published to crates.io. It was written for learning purposes and is not production-ready. See [Installation](#installation) for how to use it.

## Features

- Async request handling with Tokio
- HTTP/1.x request parsing (request line, headers, body, query params)
- Method-based routing (GET, POST, PUT, DELETE, PATCH)
- Trait-based response system with automatic JSON serialization
- Built-in response types: `OkResponse`, `BadRequestError`, `NotFoundError`, `NotImplementedError`
- Case-insensitive header management

## Installation

This library is not published on crates.io. It was created for learning purposes only and is not in a production-ready state.

To use it locally, clone the repository and reference it as a path dependency in your `Cargo.toml`:

```bash
git clone https://github.com/<your-username>/http-server.git
```

Then in your project's `Cargo.toml`:

```toml
[dependencies]
http-server = { path = "../http-server" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use http_server::{Router, Server, responses::{BadRequestError, OkResponse}};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();

    router.get(
        "/",
        Arc::new(|req, _| {
            if let Some(value) = req.query().get("error") {
                if value == "true" {
                    return BadRequestError::with_message("Bad request example").into();
                }
            }

            OkResponse::from("Hello, World!").into()
        }),
    );

    let server = Server::new("127.0.0.1:8080", router);
    println!("Listening on http://127.0.0.1:8080");
    server.serve().await?;

    Ok(())
}
```

Run the included example:

```bash
cargo run --example hello_world
```

## Project Structure

```text
src/
├── lib.rs
├── headers/          # Header parsing and storage
├── request/          # HTTP request parsing (state machine)
├── response/         # Response struct and status codes
└── server/
    ├── server.rs     # TCP listener and connection handling
    ├── router.rs     # Method + path routing
    ├── handler.rs    # EndpointHandler type alias
    └── responses/    # Built-in response and error types
```

## Documentation

More detailed documentation is available in the [`docs/`](docs/) folder:

- [API Reference](docs/api-reference.md) — All public types, structs, and functions
- [Architecture](docs/architecture.md) — Request lifecycle and internal design

## License

This project is for educational purposes. No license is provided.
