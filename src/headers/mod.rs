//! HTTP header parsing and storage.
//!
//! The [`Headers`] type provides case-insensitive storage shared by both
//! requests and responses.

mod headers;
pub(crate) mod keys;

pub use headers::Headers;
