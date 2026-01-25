use std::sync::Arc;

use crate::{
    request::Request,
    response::{Response, StatusCode},
};
use tokio::{io::AsyncWriteExt, net::TcpListener};

pub type Handler = Arc<dyn Fn(&Request, &mut Response) -> Result<(), String> + Send + Sync + 'static>;

pub struct Server {
    addr: String,
    handler: Handler,
}

impl Server {
    pub fn new(addr: &str, handler: Handler) -> Self {
        Server {
            addr: addr.to_string(),
            handler,
        }
    }

    pub async fn serve(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(&self.addr).await?;

        loop {
            let (mut stream, _) = listener.accept().await?;

            let handler = self.handler.clone();

            tokio::spawn(async move {
                let mut response = Response::new();

                match Request::from_async_reader(&mut stream).await {
                    Ok(request) => {
                        if let Err(_) = (handler)(&request, &mut response) {
                            response.set_status_code(StatusCode::InternalServerError);
                        }
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
