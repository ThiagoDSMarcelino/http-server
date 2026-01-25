#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}
