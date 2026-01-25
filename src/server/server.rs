use crate::{request::Request, response::Response};
use std::{io::Write, net::TcpListener};

pub struct Server {
    listener: TcpListener,
    closed: bool,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", host, port))?;
        Ok(Server {
            listener,
            closed: false,
        })
    }

    pub fn serve(&self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            if self.closed {
                break;
            }

            let mut stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                    continue;
                }
            };

            let request = Request::from_reader(&mut stream)?;

            println!(
                "Received a request: {} {}",
                request.get_method(),
                request.get_path()
            );
            println!("{:?}", request.get_headers());
            println!("Body: {:?}", String::from_utf8_lossy(&request.get_body()));

            let mut response = Response::new();

            response.set_default_headers();
            response.write_response(&mut stream)?;

            stream.flush()?;
        }

        Ok(())
    }
}
