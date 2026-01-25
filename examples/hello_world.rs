// cargo run --example hello_world

use http_server::{Router, Server};

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();

    router.get(
        "/",
        Arc::new(|_, res| {
            res.json("Hello, World!");
            Ok(())
        }),
    );

    let addr = "127.0.0.1:8080";

    let server = Server::new(addr, router);

    println!("Server is running on {}", addr);
    server.serve().await?;

    Ok(())
}
