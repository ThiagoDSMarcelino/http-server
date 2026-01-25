#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    Ok = 200,
}

impl StatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
        }
    }
}
