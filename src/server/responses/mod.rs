//! Built-in response and error types returned from handlers.
//!
//! Every type here can be turned into a boxed response with `.into()` and
//! returned from an [`EndpointHandler`](crate::EndpointHandler). Successful
//! responses live alongside error types such as [`BadRequestError`],
//! [`NotFoundError`], and [`NotImplementedError`].

mod client_error;
mod http_error;
mod http_response;
mod informational;
mod server_error;
mod successful;

pub(crate) use http_response::HttpResponse;

pub use client_error::*;
pub use server_error::*;
pub use successful::*;
