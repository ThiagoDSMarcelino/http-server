use serde::Serialize;

use crate::response::StatusCode;

pub trait HttpError: Sync + Send {
    fn message(&self) -> &str;
    fn status_code(&self) -> StatusCode;
}

#[derive(Serialize, Debug)]
pub(crate) struct HttpErrorResponse {
    error: String,
    message: String,
    status_code: u16,
}

impl HttpErrorResponse {
    pub fn new(error: &str, status_code: StatusCode) -> Self {
        HttpErrorResponse {
            error: error.to_string(),
            message: status_code.as_str().to_string(),
            status_code: status_code as u16,
        }
    }
}

impl dyn HttpError {
    pub(crate) fn json_response(&self) -> HttpErrorResponse {
        HttpErrorResponse::new(self.message(), self.status_code())
    }
}

impl<T: HttpError + 'static> From<T> for Box<dyn HttpError> {
    fn from(err: T) -> Self {
        Box::new(err)
    }
}
