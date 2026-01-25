use crate::StatusCode;

pub trait HttpResponse {
    fn into_response(self: Box<Self>) -> Vec<u8>;
    fn status_code(&self) -> StatusCode;
}

impl<T: HttpResponse + 'static> From<T> for Box<dyn HttpResponse> {
    fn from(result: T) -> Self {
        Box::new(result)
    }
}