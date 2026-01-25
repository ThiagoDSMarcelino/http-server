use serde::Serialize;

use crate::{response::StatusCode, responses::http_response::HttpResponse};

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

impl<T> HttpResponse for T
where
    T: HttpError,
{
    fn into_response(self: Box<Self>) -> Vec<u8> {
        // TODO: Handle serialization errors properly
        serde_json::to_vec(&HttpErrorResponse::new(self.message(), self.status_code()))
            .unwrap_or_else(|_| "Error serializing".to_string().into_bytes())
    }

    fn status_code(&self) -> StatusCode {
        <Self as HttpError>::status_code(self)
    }
}
