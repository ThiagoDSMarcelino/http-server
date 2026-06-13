use std::sync::Arc;

use crate::{Request, Response, responses::HttpResponse};

// TODO: Maybe would be better to send a specific struct instead of using Request and Response directly
// For handler cookies and other things

/// A route handler.
///
/// Receives a reference to the parsed [`Request`] and a mutable reference to the
/// [`Response`] (so headers can be customized), and returns a boxed response.
/// Wrapped in an [`Arc`] so handlers can be shared across connection tasks.
pub type EndpointHandler =
    Arc<dyn Fn(&Request, &mut Response) -> Box<dyn HttpResponse> + Send + Sync + 'static>;
