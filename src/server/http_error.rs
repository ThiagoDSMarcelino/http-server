use crate::response::StatusCode;

pub trait HttpError: Sync + Send {
    fn message(&self) -> &str;
    fn status_code(&self) -> StatusCode;
}

impl dyn HttpError {
    pub(crate) fn json_response(&self) -> String {
        format!(
            r#"{{"error": {{"message": "{}", "status_code": {}}}}}"#,
            self.message(),
            self.status_code() as u16
        )
    }
}

impl<T: HttpError + 'static> From<T> for Box<dyn HttpError> {
    fn from(err: T) -> Self {
        Box::new(err)
    }
}

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
