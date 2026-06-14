// cargo run --example static_files

use http_server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";

    // Serve every file found under the given directory. A request for "/"
    // resolves to "index.html" inside it.
    let server = Server::from_dir(addr, "examples/public");

    println!("Serving ./examples/public on http://{}", addr);
    server.serve().await?;

    Ok(())
}
