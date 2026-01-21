use super::request_state::RequestState;

use std::io;

pub struct Request {
    pub method: String,
    pub path: String,
    state: RequestState,
}

const BUFFER_SIZE: usize = 4096;

impl Request {
    fn new() -> Self {
        Request {
            method: String::new(),
            path: String::new(),
            state: RequestState::StateInit,
        }
    }

    fn done(&self) -> bool {
        self.state == RequestState::StateDone
    }

    fn parse(&mut self, buffer: &[u8]) -> Result<usize, std::io::Error> {
        self.state = RequestState::StateDone;
        Ok(buffer.len())
    }

    pub fn from_reader(mut reader: Box<dyn io::Read>) -> Result<Self, std::io::Error> {
        let mut request = Request::new();
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut len = 0;

        while !request.done() {
            let read_len = match reader.read(&mut buffer[len..]) {
                Ok(n) => n,
                Err(e) => return Err(e),
            };

            len += read_len;

            let processed_len = request.parse(&buffer[..len])?;

            buffer.copy_within(processed_len..len, 0);
            len -= processed_len;
        }

        return Ok(request);
    }
}
