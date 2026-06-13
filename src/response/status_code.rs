use serde::Serialize;

/// HTTP status codes supported by the server.
#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq)]
pub enum StatusCode {
    /// `200 OK`
    Ok = 200,
    /// `400 Bad Request`
    BadRequest = 400,
    /// `404 Not Found`
    NotFound = 404,
    /// `500 Internal Server Error`
    InternalServerError = 500,
    /// `501 Not Implemented`
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
