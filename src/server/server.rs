use crate::request::Request;
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
                    continue; // Use 'continue' em vez de 'break' para não derrubar o servidor por 1 erro
                }
            };

            let stream_reader = stream.try_clone()?;

            let request = Request::from_reader(Box::new(stream_reader))?;

            println!(
                "Received a request: {} {}",
                request.get_method(),
                request.get_path()
            );
            println!("{:?}", request.get_headers());
            println!("Body: {:?}", String::from_utf8_lossy(&request.get_body()));

            let default_response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";

            stream.write_all(default_response)?;
            stream.flush()?;
        }

        Ok(())
    }
}
