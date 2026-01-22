use super::request_state::RequestState;
use crate::headers::Headers;
use crate::request_line::RequestLine;

use std::io;

pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: Headers,
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
                    self.state = RequestState::StateDone;
                }
                RequestState::StateDone => {
                    break;
                }
            }
        }

        Ok(read)
    }

    pub fn from_reader(mut reader: Box<dyn io::Read>) -> Result<Self, std::io::Error> {
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
}
