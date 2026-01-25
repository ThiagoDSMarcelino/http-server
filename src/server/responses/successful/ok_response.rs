use serde::Serialize;

use crate::{StatusCode, responses::HttpResponse};

/// Represents a 200 OK HTTP response.
pub struct OkResponse {
    data: Vec<u8>,
}

impl OkResponse {
    /// Creates a new OkResponse with no data.
    pub fn new() -> Self {
        OkResponse { data: Vec::new() }
    }

    /// Creates a new OkResponse with the given serializable data.
    pub fn from<T: Serialize>(data: T) -> Self {
        // FIXME: Handle serialization errors properly
        let serialization = serde_json::to_vec(&data)
            .unwrap_or_else(|e| format!("Serialization failed: {}", e).as_bytes().to_vec());

        OkResponse {
            data: serialization,
        }
    }
}

impl HttpResponse for OkResponse {
    fn into_response(self: Box<Self>) -> Vec<u8> {
        self.data
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::Ok
    }
}
