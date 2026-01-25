use crate::{response::StatusCode, server::errors::http_error::HttpError};

pub struct NotFoundError {
    message: String,
}

impl NotFoundError {
    pub fn new() -> Self {
        NotFoundError {
            message: StatusCode::NotFound.as_str().to_string(),
        }
    }

    pub fn with_message<S: Into<String>>(message: S) -> Self {
        NotFoundError {
            message: message.into(),
        }
    }
}

impl HttpError for NotFoundError {
    fn message(&self) -> &str {
        &self.message
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::NotFound
    }
}
