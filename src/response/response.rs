use std::{io, usize};

use crate::{headers::Headers, response::StatusCode};

const HTTP_VERSION: &str = "HTTP/1.1";

pub fn write_status_line<W: io::Write>(
    writer: &mut W,
    status_code: StatusCode,
) -> Result<(), std::io::Error> {
    let status_line = format!(
        "{} {} {}\r\n",
        HTTP_VERSION,
        status_code as u16,
        status_code.as_str()
    );
    writer.write_all(status_line.as_bytes())?;
    Ok(())
}

pub fn write_headers<W: io::Write>(
    writer: &mut W,
    headers: &Headers,
) -> Result<(), std::io::Error> {
    for (key, value) in headers.iter() {
        let header_line = format!("{}: {}\r\n", key, value);
        writer.write_all(header_line.as_bytes())?;
    }
    writer.write_all(b"\r\n")?;
    Ok(())
}

pub fn get_default_headers(content_length: usize) -> Headers {
    let mut headers = Headers::new();

    headers.set("Content-Length", &content_length.to_string());
    headers.set("Connection", "close");
    headers.set("Content-Type", "text/plain; charset=utf-8");

    return headers;
}
