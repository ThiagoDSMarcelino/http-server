mod headers;
mod request;
mod response;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::Server::new("0.0.0.0", 8080)?;

    server.serve()?;

    Ok(())
}
