use crate::{
    Router,
    request::Request,
    response::{Response, StatusCode},
};
use tokio::{io::AsyncWriteExt, net::TcpListener};

/// Represents an HTTP server.
pub struct Server {
    addr: String,
    router: Router,
}

impl Server {
    /// Creates a new Server instance with the specified address and router.
    pub fn new(addr: &str, router: Router) -> Self {
        Server {
            addr: addr.to_string(),
            router,
        }
    }

    /// Starts the server and begins listening for incoming connections.
    pub async fn serve(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&self.addr).await?;

        let routes_handler = self.router.build();

        loop {
            let (mut stream, _) = listener.accept().await?;

            let handler = routes_handler.clone();

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
