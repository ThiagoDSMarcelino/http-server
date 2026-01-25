use std::{collections::HashMap, sync::Arc};

use crate::{Handler, Request, Response, errors::NotFoundError};

pub struct Router {
    endpoints: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            endpoints: HashMap::new(),
        }
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.endpoints.insert(format!("POST {}", path), handler);
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.endpoints.insert(format!("GET {}", path), handler);
    }

    pub(crate) fn build(self) -> Handler {
        Arc::new(move |req: &Request, res: &mut Response| {
            let key = format!("{} {}", req.method(), req.path());

            if let Some(handler) = self.endpoints.get(&key) {
                return handler(req, res);
            }

            Err(NotFoundError::with_message(format!(
                "Cannot {} {}",
                req.method(),
                req.path()
            )))?
        })
    }
}
