use std::io;

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

    pub(crate) fn write_response<W: io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error> {
        let status_line = format!(
            "{} {} {}\r\n",
            HTTP_VERSION,
            self.status_code as u16,
            self.status_code.as_str()
        );
        writer.write_all(status_line.as_bytes())?;
        for (key, value) in self.headers.iter() {
            let header_line = format!("{}: {}\r\n", key, value);
            writer.write_all(header_line.as_bytes())?;
        }
        writer.write_all(b"\r\n")?;
        Ok(())
    }

    pub(crate) fn set_default_headers(&mut self) -> Headers {
        let mut headers = Headers::new();

        headers.set("Content-Length", &self.body.len().to_string());
        headers.set("Connection", "close");
        headers.set("Content-Type", "text/plain; charset=utf-8");

        return headers;
    }
}
