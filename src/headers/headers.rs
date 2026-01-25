use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct Headers {
    data: HashMap<String, String>,
}

// Theoretically, \n could be the separator as well if the first line ends with it, but for now we only support \r\n.
const LINE_SEPARATOR: &[u8] = b"\r\n";
const HEADER_SEPARATOR: u8 = b':';
const SPACE: &[u8] = b" ";

fn is_valid_key(key: &str) -> bool {
    let valid_key_chars_regex = Regex::new("^[a-zA-Z0-9!#$%&'*+.^_`|~-]+$").unwrap();

    !key.as_bytes().starts_with(SPACE)
        && !key.as_bytes().ends_with(SPACE)
        && !key.is_empty()
        && valid_key_chars_regex.is_match(key)
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            data: HashMap::new(),
        }
    }

    pub fn iter(&'_ self) -> std::collections::hash_map::Iter<'_, String, String> {
        self.data.iter()
    }

    pub fn get<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
        let local_key = key.to_lowercase();

        if let Some(value) = self.data.get(&local_key) {
            if let Ok(parsed) = value.trim().parse::<T>() {
                return Some(parsed);
            }
        }

        None
    }

    pub fn set(&mut self, key: &str, value: &str) {
        // Header field names are case-insensitive
        // https://datatracker.ietf.org/doc/html/rfc9112#name-field-syntax
        let local_key = key.to_lowercase().to_string();

        self.data
            .entry(local_key)
            .and_modify(|current| {
                current.push_str(", ");
                current.push_str(value);
            })
            .or_insert(value.to_string());
    }

    pub fn contains(&self, key: &str) -> bool {
        let local_key = key.to_lowercase();

        self.data.contains_key(&local_key)
    }

    pub(crate) fn parse(&mut self, data: &[u8]) -> Result<(bool, usize), std::io::Error> {
        let mut read: usize = 0;

        loop {
            let current_slice = &data[read..];

            let index = current_slice
                .windows(LINE_SEPARATOR.len())
                .position(|window| window == LINE_SEPARATOR);

            if index.is_none() {
                return Ok((false, read));
            }

            let line_buffer = &current_slice[..index.unwrap()];
            read += line_buffer.len() + LINE_SEPARATOR.len();

            if line_buffer.is_empty() {
                return Ok((true, read));
            }

            let parts = line_buffer
                .splitn(2, |&b| b == HEADER_SEPARATOR)
                .collect::<Vec<&[u8]>>();

            if parts.len() != 2 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid header format",
                ));
            }

            let key = String::from_utf8(parts[0].to_vec())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            if !is_valid_key(&key) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid header key",
                ));
            }

            let value = String::from_utf8(parts[1].to_vec())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
                .trim_start()
                .to_string();

            self.set(&key, &value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_single_header() {
        let mut headers = Headers::new();
        let data = b"Host: localhost:8080\r\n\r\n";

        let result = headers.parse(data);
        assert!(result.is_ok());

        let (done, consumed) = result.unwrap();
        assert!(done);
        assert_eq!(consumed, data.len());

        assert!(headers.get::<String>("Host").is_some());
        assert_eq!(headers.get::<String>("Host").unwrap(), "localhost:8080");
    }

    #[test]
    fn test_invalid_spacing_header() {
        let mut headers = Headers::new();
        let data = b"       Host : localhost:8080       \r\n\r\n";

        let result = headers.parse(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_header_field_name() {
        let mut headers = Headers::new();
        let data = b"H\xA9st: localhost:8080\r\n\r\n";

        let result = headers.parse(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_header_multiples_equals_field_names() {
        let mut headers = Headers::new();
        let data = b"Host: localhost:8080\r\nHost: localhost:8081\r\n\r\n";

        let result = headers.parse(data);
        assert!(result.is_ok());

        let (done, consumed) = result.unwrap();
        assert!(done);
        assert_eq!(consumed, data.len());

        assert!(headers.get::<String>("Host").is_some());
        assert_eq!(
            headers.get::<String>("Host").unwrap(),
            "localhost:8080, localhost:8081"
        );
    }
}
