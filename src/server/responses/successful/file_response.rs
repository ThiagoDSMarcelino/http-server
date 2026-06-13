use crate::{StatusCode, responses::HttpResponse};

/// Represents a 200 OK HTTP response that serves the raw contents of a file.
pub struct FileResponse {
    data: Vec<u8>,
    content_type: String,
}

impl FileResponse {
    /// Creates a new FileResponse with the given bytes and Content-Type.
    pub fn new<S: Into<String>>(data: Vec<u8>, content_type: S) -> Self {
        FileResponse {
            data,
            content_type: content_type.into(),
        }
    }
}

impl HttpResponse for FileResponse {
    fn into_response(self: Box<Self>) -> Vec<u8> {
        self.data
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::Ok
    }

    fn content_type(&self) -> Option<&str> {
        Some(&self.content_type)
    }
}
