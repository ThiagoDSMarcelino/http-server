use std::io;

pub(crate) struct RequestLine {
    method: String,
    path: String,
    version: String,
}

// Theoretically, \n could be the separator as well if the first line ends with it, but for now we only support \r\n.
const LINE_SEPARATOR: &[u8] = b"\r\n";

fn is_valid_method(method: &str) -> bool {
    matches!(
        method,
        "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" | "PATCH" | "TRACE" | "CONNECT"
    )
}

fn is_valid_version(version: &str) -> bool {
    matches!(version, "HTTP/1.0" | "HTTP/1.1" | "HTTP/2.0")
}

impl RequestLine {
    pub fn parse(data: &[u8]) -> Result<Option<(Self, usize)>, std::io::Error> {
        let index = data
            .windows(LINE_SEPARATOR.len())
            .position(|window| window == LINE_SEPARATOR);

        if index.is_none() {
            return Ok(None);
        }

        let request_line_buffer = &data[..index.unwrap()];

        let parts = request_line_buffer
            .split(|&b| b == b' ')
            .collect::<Vec<&[u8]>>();

        if parts.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid request line format",
            ));
        }

        let method = String::from_utf8(parts[0].to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if !is_valid_method(&method) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid HTTP method",
            ));
        }

        let path = String::from_utf8(parts[1].to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let version = String::from_utf8(parts[2].to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if !is_valid_version(&version) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid HTTP version",
            ));
        }

        let bytes_consumed = index.unwrap() + LINE_SEPARATOR.len();
        let request_line = RequestLine {
            method,
            path,
            version,
        };

        Ok(Some((request_line, bytes_consumed)))
    }

    pub fn get_method(&self) -> &str {
        &self.method
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }
}
