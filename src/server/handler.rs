use std::sync::Arc;

use crate::{Request, Response, responses::HttpResponse};

// TODO: Maybe would be better to send a specific struct instead of using Request and Response directly
// For handler cookies and other things
pub type EndpointHandler =
    Arc<dyn Fn(&Request, &mut Response) -> Box<dyn HttpResponse> + Send + Sync + 'static>;
