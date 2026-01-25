use tokio::io::AsyncWriteExt;

use crate::{headers::Headers, response::StatusCode};

pub struct Response {
    body: Vec<u8>,
    headers: Headers,
    status_code: StatusCode,
}

const HTTP_VERSION: &str = "HTTP/1.1";

impl Response {
    pub fn new() -> Self {
        Response {
            body: Vec::new(),
            headers: Headers::new(),
            status_code: StatusCode::Ok,
        }
    }

    pub(crate) async fn write_response<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        // Write status line
        let status_line = format!(
            "{} {} {}\r\n",
            HTTP_VERSION,
            self.status_code as u16,
            self.status_code.as_str()
        );
        writer.write_all(status_line.as_bytes()).await?;

        // Write headers
        for (key, value) in self.headers.iter() {
            let header_line = format!("{}: {}\r\n", key, value);
            writer.write_all(header_line.as_bytes()).await?;
        }
        writer.write_all(b"\r\n").await?;

        // Write body
        writer.write_all(&self.body).await?;

        Ok(())
    }

    pub(crate) fn set_default_headers(&mut self) {
        self.headers.set("Content-Length", &self.body.len().to_string());
        self.headers.set("Connection", "close");
        self.headers.set("Content-Type", "text/plain; charset=utf-8");
    }
}
