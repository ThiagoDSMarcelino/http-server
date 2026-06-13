use crate::StatusCode;

pub trait HttpResponse {
    fn into_response(self: Box<Self>) -> Vec<u8>;
    fn status_code(&self) -> StatusCode;

    /// Returns the Content-Type for this response, if it differs from the
    /// server default. Returning `None` lets the server pick its default.
    fn content_type(&self) -> Option<&str> {
        None
    }
}

impl<T: HttpResponse + 'static> From<T> for Box<dyn HttpResponse> {
    fn from(result: T) -> Self {
        Box::new(result)
    }
}