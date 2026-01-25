use std::{collections::HashMap, sync::Arc};

use crate::{EndpointHandler, Request, Response, responses::NotFoundError};

pub struct Router {
    endpoints: HashMap<String, EndpointHandler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            endpoints: HashMap::new(),
        }
    }

    pub fn post(&mut self, path: &str, handler: EndpointHandler) {
        self.endpoints.insert(format!("POST {}", path), handler);
    }

    pub fn get(&mut self, path: &str, handler: EndpointHandler) {
        self.endpoints.insert(format!("GET {}", path), handler);
    }

    pub fn delete(&mut self, path: &str, handler: EndpointHandler) {
        self.endpoints.insert(format!("DELETE {}", path), handler);
    }

    pub fn put(&mut self, path: &str, handler: EndpointHandler) {
        self.endpoints.insert(format!("PUT {}", path), handler);
    }

    pub fn patch(&mut self, path: &str, handler: EndpointHandler) {
        self.endpoints.insert(format!("PATCH {}", path), handler);
    }

    pub(crate) fn build(self) -> EndpointHandler {
        Arc::new(move |req: &Request, res: &mut Response| {
            let key = format!("{} {}", req.method(), req.path());

            if let Some(handler) = self.endpoints.get(&key) {
                return handler(req, res);
            }

            let error =
                NotFoundError::with_message(format!("Cannot {} {}", req.method(), req.path()));
            return error.into();
        })
    }
}
