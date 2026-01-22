mod request;
mod server;
mod request_line;

fn main() {
    let server = server::Server::new("0.0.0.0", 8080).unwrap();

    server.serve().unwrap();
}
