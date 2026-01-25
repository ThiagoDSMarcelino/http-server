use std::{collections::HashMap, io};

pub(super) struct RequestLine {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) query: HashMap<String, String>,
    pub(crate) version: String,
}

// Theoretically, \n could be the separator as well if the first line ends with it, but for now we only support \r\n.
const LINE_SEPARATOR: &[u8] = b"\r\n";
const QUERY_SEPARATOR: char = '?';
const PARAMETER_SEPARATOR: char = '&';

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

        let target = String::from_utf8(parts[1].to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let (path, _query) = match target.find(QUERY_SEPARATOR) {
            Some(pos) => (&target[..pos], &target[pos + 1..]),
            None => (target.as_str(), ""),
        };

        let mut query = HashMap::new();
        for param in _query.split(PARAMETER_SEPARATOR) {
            if param.is_empty() {
                continue;
            }

            let mut key_value = param.splitn(2, '=');
            let key = key_value.next().unwrap_or("");
            if key.is_empty() {
                continue;
            }

            let value = key_value.next().unwrap_or("").to_string();
            query.insert(key.to_string(), value);
        }

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
            path: path.to_string(),
            query,
            version,
        };

        Ok(Some((request_line, bytes_consumed)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_get_request_line() {
        let request_line_result = RequestLine::parse(b"GET / HTTP/1.1\r\n\r\n");

        assert!(request_line_result.is_ok());

        let request_line_result = request_line_result.unwrap();

        assert!(request_line_result.is_some());

        let (request_line, consumed) = request_line_result.unwrap();

        assert_eq!(consumed, 16);

        assert_eq!(request_line.method, "GET");
        assert_eq!(request_line.path, "/");
        assert_eq!(request_line.version, "HTTP/1.1");
    }

    #[test]
    fn test_good_get_request_line_with_path() {
        let request_line_result = RequestLine::parse(b"GET /coffee HTTP/1.1\r\n\r\n");

        assert!(request_line_result.is_ok());

        let request_line_result = request_line_result.unwrap();

        assert!(request_line_result.is_some());

        let (request_line, consumed) = request_line_result.unwrap();

        assert_eq!(consumed, 22);

        assert_eq!(request_line.method, "GET");
        assert_eq!(request_line.path, "/coffee");
        assert_eq!(request_line.version, "HTTP/1.1");
    }

    #[test]
    fn test_invalid_number_of_parts_in_request_line() {
        let request_line_result = RequestLine::parse(b"coffee HTTP/1.1\r\n\r\n");

        assert!(request_line_result.is_err());
    }
}
