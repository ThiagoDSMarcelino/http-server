// cargo run --example hello_world

use http_server::{Handler, Server};

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler: Handler = Arc::new(|req, _| {
        println!("Handling request for {}", req.uri());
        
        Ok(())
    });

    let addr = "127.0.0.1:8080";

    let server = Server::new(addr, handler);

    println!("Server is running on {}", addr);
    server.serve().await?;

    Ok(())
}
