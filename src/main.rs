mod request;
mod server;
mod request_line;
mod headers;
mod body;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::Server::new("0.0.0.0", 8080)?;

    server.serve()?;

    Ok(())
}
