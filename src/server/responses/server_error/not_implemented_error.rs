use crate::{response::StatusCode, responses::http_error::HttpError, };

/// Represents a 501 Not Implemented HTTP error.
pub struct NotImplementedError {
    message: String,
}

impl NotImplementedError {
    /// Creates a new NotImplementedError with the default message.
    pub fn new() -> Self {
        NotImplementedError {
            message: StatusCode::NotImplemented.as_str().to_string(),
        }
    }

    /// Creates a new NotImplementedError with a custom message.
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
