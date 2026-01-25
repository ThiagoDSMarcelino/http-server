use crate::{response::StatusCode, responses::http_error::HttpError};

/// Represents a 404 Not Found HTTP error.
pub struct NotFoundError {
    message: String,
}

impl NotFoundError {
    /// Creates a new NotFoundError with the default message.
    pub fn new() -> Self {
        NotFoundError {
            message: StatusCode::NotFound.as_str().to_string(),
        }
    }

    /// Creates a new NotFoundError with a custom message.
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
