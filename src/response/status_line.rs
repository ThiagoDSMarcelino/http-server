use std::io;

use crate::response::StatusCode;

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
