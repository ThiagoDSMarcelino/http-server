use std::sync::Arc;

use crate::{Request, Response, results::HttpResult};

// FIXME: Maybe would be better to send a specific struct instead of using Request and Response directly
// For handler cookies and other things
pub type EndpointHandler =
    Arc<dyn Fn(&Request, &mut Response) -> Box<dyn HttpResult> + Send + Sync + 'static>;
