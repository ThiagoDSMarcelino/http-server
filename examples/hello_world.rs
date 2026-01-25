// cargo run --example hello_world

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

    let addr = "127.0.0.1:8080";

    let server = Server::new(addr, router);

    println!("Server is running on {}", addr);
    server.serve().await?;

    Ok(())
}
