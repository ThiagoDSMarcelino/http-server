use crate::{response::StatusCode, results::http_error::HttpError};

pub struct BadRequestError {
    message: String,
}

impl BadRequestError {
    pub fn new() -> Self {
        BadRequestError {
            message: StatusCode::BadRequest.as_str().to_string(),
        }
    }

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
