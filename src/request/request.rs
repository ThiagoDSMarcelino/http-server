use std::collections::HashMap;

use tokio::io::AsyncReadExt;

use crate::headers::{self, Headers};

use super::body;
use super::request_line::RequestLine;
use super::request_state::RequestState;

pub struct Request {
    method: String,
    path: String,
    version: String,
    query: HashMap<String, String>,
    headers: Headers,
    body: Vec<u8>,
    state: RequestState,
}

const BUFFER_SIZE: usize = 4096;

impl Request {
    fn new() -> Self {
        Request {
            method: String::new(),
            path: String::new(),
            version: String::new(),
            query: HashMap::new(),
            headers: Headers::new(),
            body: Vec::new(),
            state: RequestState::StateInit,
        }
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    fn done(&self) -> bool {
        self.state == RequestState::StateDone
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn http_version(&self) -> &str {
        &self.version
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    fn set_request_line(&mut self, rl: RequestLine) {
        self.method = rl.method;
        self.path = rl.path;
        self.version = rl.version;
        self.query = rl.query;
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

                    self.set_request_line(rl);

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
                        if let Some(cl) = self
                            .headers
                            .get::<usize>(headers::keys::CONTENT_LENGTH_HEADER)
                        {
                            self.body.reserve(cl);
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
                    let content_length = self
                        .headers
                        .get::<usize>(headers::keys::CONTENT_LENGTH_HEADER);

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

    pub(crate) async fn from_reader<R: tokio::io::AsyncRead + Unpin>(
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

    impl tokio::io::AsyncRead for ChunkReader<'_> {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<io::Result<()>> {
            let me = self.get_mut();

            if me.pos >= me.data.len() {
                return Poll::Ready(Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "EOF",
                )));
            }

            let end_index = std::cmp::min(me.pos + me.num_bytes_per_read, me.data.len());
            let available_chunk = end_index - me.pos;

            let n = std::cmp::min(buf.remaining(), available_chunk);

            buf.put_slice(&me.data[me.pos..me.pos + n]);

            me.pos += n;

            Poll::Ready(Ok(()))
        }
    }

    use std::{io, task::Poll};

    use super::*;

    #[tokio::test]
    async fn test_good_get_request_receiving_under_size_buffer() {
        let reader = ChunkReader::new(b"GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n", 8);
        let request_result = Request::from_reader(reader).await;

        assert!(request_result.is_ok());

        let request = request_result.unwrap();

        assert_eq!(request.method(), "GET");
        assert_eq!(request.path(), "/");
        assert_eq!(request.http_version(), "HTTP/1.1");

        assert!(request.headers().get::<String>("Host").is_some());
        assert_eq!(
            request.headers().get::<String>("Host").unwrap(),
            "localhost:8080"
        );

        assert!(request.headers().get::<String>("User-Agent").is_some());
        assert_eq!(
            request.headers().get::<String>("User-Agent").unwrap(),
            "curl/7.81.0"
        );

        assert!(request.headers().get::<String>("Accept").is_some());
        assert_eq!(request.headers().get::<String>("Accept").unwrap(), "*/*");
    }

    #[tokio::test]
    async fn test_good_get_request_receiving_over_size_buffer() {
        let reader = ChunkReader::new(b"GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.81.0\r\nAccept: */*\r\n\r\n", 1024);
        let request_result = Request::from_reader(reader).await;

        assert!(request_result.is_ok());

        let request = request_result.unwrap();

        assert_eq!(request.method(), "GET");
        assert_eq!(request.path(), "/");
        assert_eq!(request.http_version(), "HTTP/1.1");

        assert!(request.headers().get::<String>("Host").is_some());
        assert_eq!(
            request.headers().get::<String>("Host").unwrap(),
            "localhost:8080"
        );

        assert!(request.headers().get::<String>("User-Agent").is_some());
        assert_eq!(
            request.headers().get::<String>("User-Agent").unwrap(),
            "curl/7.81.0"
        );

        assert!(request.headers().get::<String>("Accept").is_some());
        assert_eq!(request.headers().get::<String>("Accept").unwrap(), "*/*");
    }
}
