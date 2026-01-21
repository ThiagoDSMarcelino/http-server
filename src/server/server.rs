use std::net::TcpListener;
use crate::request::Request;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", host, port))?;
        Ok(Server { listener })
    }

    pub fn serve(&self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(_stream) => {
                    let request = Request::from_reader(Box::new(_stream))?;

                    println!("Received a request: {} {}", request.method, request.path);
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }
}
