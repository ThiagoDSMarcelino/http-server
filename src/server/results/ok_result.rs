use serde::Serialize;

use crate::{StatusCode, results::HttpResult};

pub struct OkResult {
    data: Vec<u8>,
}

impl OkResult {
    pub fn new<T: Serialize>(data: T) -> Self {
        // TODO: Handle serialization errors properly
        let serialization = serde_json::to_vec(&data)
            .map_err(|e| format!("Serialization failed: {}", e).as_bytes().to_vec())
            .unwrap_or_else(|err_bytes| err_bytes);

        OkResult {
            data: serialization,
        }
    }
}

impl HttpResult for OkResult {
    fn into_response(self: Box<Self>) -> Vec<u8> {
        self.data
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::Ok
    }
}
