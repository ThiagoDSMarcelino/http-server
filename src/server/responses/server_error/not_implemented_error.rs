use crate::{response::StatusCode, responses::http_error::HttpError, };

pub struct NotImplementedError {
    message: String,
}

impl NotImplementedError {
    pub fn new() -> Self {
        NotImplementedError {
            message: StatusCode::NotImplemented.as_str().to_string(),
        }
    }

    pub fn with_message<S: Into<String>>(message: S) -> Self {
        NotImplementedError {
            message: message.into(),
        }
    }
}

impl HttpError for NotImplementedError {
    fn message(&self) -> &str {
        &self.message
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::NotImplemented
    }
}
