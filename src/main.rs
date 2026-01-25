use std::sync::Arc;

use crate::server::Server;

mod headers;
mod request;
mod response;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler: server::Handler = Arc::new(|| {
        println!("Handling request");
        Ok(())
    });

    let server = Server::new("0.0.0.0:8080", handler);

    println!("Server is running on 0.0.0.0:8080");
    server.serve().await?;

    Ok(())
}
