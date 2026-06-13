//! A lightweight, async HTTP server library built on top of [Tokio].
//!
//! `http-server` was written as a learning project to explore async I/O, TCP
//! streams, HTTP parsing, and trait-based response systems. It provides
//! method-based routing, static file serving, and a small set of built-in
//! response and error types with automatic JSON serialization.
//!
//! > **Note:** This library is not published on crates.io and is not
//! > production-ready.
//!
//! # Quick start
//!
//! Register handlers on a [`Router`] and hand it to a [`Server`]:
//!
//! ```no_run
//! use http_server::{Router, Server, responses::{BadRequestError, OkResponse}};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut router = Router::new();
//!
//!     router.get(
//!         "/",
//!         Arc::new(|req, _| {
//!             if req.query().get("error").map(|v| v == "true").unwrap_or(false) {
//!                 return BadRequestError::with_message("Bad request example").into();
//!             }
//!
//!             OkResponse::from("Hello, World!").into()
//!         }),
//!     );
//!
//!     let server = Server::new("127.0.0.1:8080", router);
//!     server.serve().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Serving static files
//!
//! Instead of a router, a [`Server`] can serve files straight from a directory.
//! A request for `/` resolves to `index.html`, the `Content-Type` is inferred
//! from each file's extension, and paths that try to escape the directory
//! (e.g. using `..`) are rejected with a `404`:
//!
//! ```no_run
//! use http_server::Server;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Serve every file found under ./public
//!     let server = Server::from_dir("127.0.0.1:8080", "public");
//!     server.serve().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! [Tokio]: https://tokio.rs/

#![warn(missing_docs)]

pub mod headers;
mod request;
mod response;
mod server;

pub use request::Request;
pub use response::*;
pub use server::*;

pub use server::responses;
