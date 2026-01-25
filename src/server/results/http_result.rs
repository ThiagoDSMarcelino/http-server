use crate::StatusCode;

pub trait HttpResult {
    fn into_response(self: Box<Self>) -> Vec<u8>;
    fn status_code(&self) -> StatusCode;
}

impl<T: HttpResult + 'static> From<T> for Box<dyn HttpResult> {
    fn from(result: T) -> Self {
        Box::new(result)
    }
}