use serde::Serialize;

#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
}

impl StatusCode {
    /// Returns the standard reason phrase for the status code.
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
        }
    }
}
