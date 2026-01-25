use std::sync::Arc;

use crate::{Request, Response, errors::HttpError};

pub type HttpResult<T> = Result<T, Box<dyn HttpError>>;
pub type Handler = Arc<dyn Fn(&Request, &mut Response) -> HttpResult<()> + Send + Sync + 'static>;
