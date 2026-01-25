use crate::{response::StatusCode, responses::http_error::HttpError};

/// Represents a 400 Bad Request HTTP error.
pub struct BadRequestError {
    message: String,
}

impl BadRequestError {
    /// Creates a new BadRequestError with the default message.
    pub fn new() -> Self {
        BadRequestError {
            message: StatusCode::BadRequest.as_str().to_string(),
        }
    }

    /// Creates a new BadRequestError with a custom message.
    pub fn with_message<S: Into<String>>(message: S) -> Self {
        BadRequestError {
            message: message.into(),
        }
    }
}

impl HttpError for BadRequestError {
    fn message(&self) -> &str {
        &self.message
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BadRequest
    }
}
