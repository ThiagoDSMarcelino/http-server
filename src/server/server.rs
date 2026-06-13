use std::path::PathBuf;

use crate::{
    EndpointHandler, FileServer, Router,
    request::Request,
    response::{Response, StatusCode},
};
use tokio::{io::AsyncWriteExt, net::TcpListener};

/// Represents an HTTP server.
pub struct Server {
    addr: String,
    handler: EndpointHandler,
}

impl Server {
    /// Creates a new Server instance that dispatches requests through a router.
    pub fn new(addr: &str, router: Router) -> Self {
        Server {
            addr: addr.to_string(),
            handler: router.build(),
        }
    }

    /// Creates a new Server instance that serves static files from a directory.
    pub fn from_dir<P: Into<PathBuf>>(addr: &str, dir: P) -> Self {
        Server {
            addr: addr.to_string(),
            handler: FileServer::new(dir).build(),
        }
    }

    /// Starts the server and begins listening for incoming connections.
    pub async fn serve(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&self.addr).await?;

        let request_handler = self.handler;

        loop {
            let (mut stream, _) = listener.accept().await?;

            let handler = request_handler.clone();

            tokio::spawn(async move {
                let mut response = Response::new();

                match Request::from_reader(&mut stream).await {
                    Ok(request) => {
                        let result = (handler)(&request, &mut response);
                        response.set_result(result);
                    }
                    Err(_) => {
                        response.set_status_code(StatusCode::BadRequest);
                    }
                }

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
