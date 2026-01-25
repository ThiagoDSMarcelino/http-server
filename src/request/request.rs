use tokio::io::AsyncReadExt;

use crate::headers::Headers;

use super::body;
use super::request_line::RequestLine;
use super::request_state::RequestState;

use std::io;

pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: Headers,
    body: Vec<u8>,
    state: RequestState,
}

const BUFFER_SIZE: usize = 4096;

const CONTENT_LENGTH_HEADER: &str = "Content-Length";

impl Request {
    fn new() -> Self {
        Request {
            method: String::new(),
            path: String::new(),
            version: String::new(),
            headers: Headers::new(),
            body: Vec::new(),
            state: RequestState::StateInit,
        }
    }

    pub fn get_headers(&self) -> &Headers {
        &self.headers
    }

    fn done(&self) -> bool {
        self.state == RequestState::StateDone
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

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    fn parse(&mut self, buffer: &[u8]) -> Result<usize, std::io::Error> {
        let mut read: usize = 0;

        loop {
            let current_slice = &buffer[read..];

            match self.state {
                RequestState::StateInit => {
                    self.state = RequestState::StateRequestLine;
                }
                RequestState::StateRequestLine => {
                    let request_line_data = RequestLine::parse(current_slice)?;

                    if request_line_data.is_none() {
                        break;
                    }

                    let (rl, consumed) = request_line_data.unwrap();

                    self.method = rl.get_method().to_string();
                    self.path = rl.get_path().to_string();
                    self.version = rl.get_version().to_string();

                    self.state = RequestState::StateHeaders;

                    read += consumed;
                }
                RequestState::StateHeaders => {
                    let (done, consumed) = self.headers.parse(current_slice)?;

                    read += consumed;

                    if done {
                        // FIXME: Improve body detection
                        // Technically, there are cases where a body can be present without
                        // a Content-Length header (e.g., Transfer-Encoding: chunked), but
                        // for simplicity, we only check for Content-Length here.
                        if self.headers.contains(CONTENT_LENGTH_HEADER) {
                            self.state = RequestState::StateBody;
                        } else {
                            self.state = RequestState::StateDone;
                        }

                        continue;
                    }

                    if consumed == 0 {
                        break;
                    }
                }
                RequestState::StateBody => {
                    let content_length = self.headers.get::<usize>(CONTENT_LENGTH_HEADER);

                    if content_length.is_none() {
                        self.state = RequestState::StateDone;
                        continue;
                    }

                    let (done, consumed) =
                        body::parse(&mut self.body, current_slice, content_length.unwrap())?;

                    if done {
                        self.state = RequestState::StateDone;
                        continue;
                    }

                    if consumed == 0 {
                        break;
                    }

                    read += consumed;
                }
                RequestState::StateDone => {
                    break;
                }
            }
        }

        Ok(read)
    }

    #[allow(dead_code)]
    fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, std::io::Error> {
        let mut request = Request::new();
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut len = 0;

        while !request.done() {
            let read_len = reader.read(&mut buffer[len..])?;

            len += read_len;

            let processed_len = request.parse(&buffer[..len])?;

            buffer.copy_within(processed_len..len, 0);
            len -= processed_len;
        }

        return Ok(request);
    }

    pub(crate) async fn from_async_reader<R: tokio::io::AsyncRead + Unpin>(
        mut reader: R,
    ) -> Result<Self, std::io::Error> {
        let mut request = Request::new();
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut len = 0;

        while !request.done() {
            let read_len = reader.read(&mut buffer[len..]).await?;

            len += read_len;

            let processed_len = request.parse(&buffer[..len])?;

            buffer.copy_within(processed_len..len, 0);
            len -= processed_len;
        }

        return Ok(request);
    }
}

#[cfg(test)]
mod tests {

    struct ChunkReader<'a> {
        data: &'a [u8],
        num_bytes_per_read: usize,
        pos: usize,
    }

    impl<'a> ChunkReader<'a> {
        fn new(data: &'a [u8], num_bytes_per_read: usize) -> Self {
            ChunkReader {
                data,
                num_bytes_per_read,
                pos: 0,
            }
        }
    }

    impl io::Read for ChunkReader<'_> {
        // Read reads up to len(p) or numBytesPerRead bytes from the string per call
        // its useful for simulating reading a variable number of bytes per chunk from a network connection
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
            if self.pos >= self.data.len() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "EOF",
                ));
            }

            let end_index = std::cmp::min(self.pos + self.num_bytes_per_read, self.data.len());
            let available_chunk = end_index - self.pos;
            let n = std::cmp::min(buf.len(), available_chunk);

            buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;

            Ok(n)
        }
    }

    use super::*;

    #[test]
    fn test_good_get_request_receiving_under_size_buffer() {
        let reader = ChunkReader::new(b"GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n", 8);
        let request_result = Request::from_reader(reader);

        assert!(request_result.is_ok());

        let request = request_result.unwrap();

        assert_eq!(request.get_method(), "GET");
        assert_eq!(request.get_path(), "/");
        assert_eq!(request.get_version(), "HTTP/1.1");

        assert!(request.get_headers().get::<String>("Host").is_some());
        assert_eq!(
            request.get_headers().get::<String>("Host").unwrap(),
            "localhost:8080"
        );

        assert!(request.get_headers().get::<String>("User-Agent").is_some());
        assert_eq!(
            request.get_headers().get::<String>("User-Agent").unwrap(),
            "curl/7.81.0"
        );

        assert!(request.get_headers().get::<String>("Accept").is_some());
        assert_eq!(
            request.get_headers().get::<String>("Accept").unwrap(),
            "*/*"
        );
    }

    #[test]
    fn test_good_get_request_receiving_over_size_buffer() {
        let reader = ChunkReader::new(b"GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n", 1024);
        let request_result = Request::from_reader(reader);

        assert!(request_result.is_ok());

        let request = request_result.unwrap();

        assert_eq!(request.get_method(), "GET");
        assert_eq!(request.get_path(), "/");
        assert_eq!(request.get_version(), "HTTP/1.1");

        assert!(request.get_headers().get::<String>("Host").is_some());
        assert_eq!(
            request.get_headers().get::<String>("Host").unwrap(),
            "localhost:8080"
        );

        assert!(request.get_headers().get::<String>("User-Agent").is_some());
        assert_eq!(
            request.get_headers().get::<String>("User-Agent").unwrap(),
            "curl/7.81.0"
        );

        assert!(request.get_headers().get::<String>("Accept").is_some());
        assert_eq!(
            request.get_headers().get::<String>("Accept").unwrap(),
            "*/*"
        );
    }
}
