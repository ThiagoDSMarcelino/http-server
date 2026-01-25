use crate::{request::Request, response::Response};
use tokio::{io::AsyncWriteExt, net::TcpListener};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(host: &str, port: u16) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
        Ok(Server { listener })
    }

    pub async fn serve(&self) -> Result<(), std::io::Error> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;

            tokio::spawn(async move {
                let request = match Request::from_async_reader(&mut stream).await {
                    Ok(req) => req,
                    Err(e) => {
                        // TODO: write error response
                        eprintln!("Failed to parse request: {}", e);
                        return;
                    }
                };

                println!(
                    "Received a request: {} {}",
                    request.get_method(),
                    request.get_path()
                );
                println!("{:?}", request.get_headers());
                println!("Body: {:?}", String::from_utf8_lossy(&request.get_body()));

                let mut response = Response::new();
                response.set_default_headers();
                if let Err(err) = response.write_response(&mut stream).await {
                    eprintln!("Failed to write response: {}", err);
                }

                // Ensure all data is flushed to the stream
                if let Err(err) = stream.flush().await {
                    eprintln!("Failed to flush stream: {}", err);
                }
            });
        }
    }
}
