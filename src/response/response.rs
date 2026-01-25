use serde::Serialize;
use tokio::io::AsyncWriteExt;

use crate::{
    headers::{self, Headers},
    response::StatusCode,
};

pub struct Response {
    body: Vec<u8>,
    headers: Headers,
    status_code: StatusCode,
}

const HTTP_VERSION: &str = "HTTP/1.1";

const CONTENT_TYPE_JSON: &str = "application/json; charset=utf-8";

impl Response {
    pub fn new() -> Self {
        Response {
            body: Vec::new(),
            headers: Headers::new(),
            status_code: StatusCode::Ok,
        }
    }

    pub fn set_status_code(&mut self, status_code: StatusCode) {
        self.status_code = status_code;
    }

    pub fn json<T: Serialize>(&mut self, body: T) {
        self.headers
            .set(headers::keys::CONTENT_TYPE_KEY, CONTENT_TYPE_JSON);

        match serde_json::to_vec(&body) {
            Ok(json_body) => self.body = json_body,
            Err(err) => {
                self.status_code = StatusCode::InternalServerError;
                let error_response = serde_json::json!({
                    "error": err.to_string(),
                    "message": "Failed to serialize JSON",
                    "status_code": 500
                });
                self.body = serde_json::to_vec(&error_response).unwrap_or_default();
            }
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
        self.headers.set(
            headers::keys::CONTENT_LENGTH_HEADER,
            &self.body.len().to_string(),
        );

        self.headers.set(headers::keys::CONNECTION_HEADER, "close");

        if !self.headers.contains(headers::keys::CONTENT_TYPE_KEY) {
            self.headers
                .set(headers::keys::CONTENT_TYPE_KEY, CONTENT_TYPE_JSON);
        }
    }
}
