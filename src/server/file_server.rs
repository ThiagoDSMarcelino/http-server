use std::{
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use crate::{
    EndpointHandler, Request, Response,
    responses::{FileResponse, NotFoundError},
};

/// Serves static files from a directory on the local filesystem.
///
/// A request for `/` resolves to `index.html` inside the directory, and any
/// request path is resolved relative to the configured root. Paths that try to
/// escape the root (e.g. using `..`) are rejected with a 404.
pub struct FileServer {
    root: PathBuf,
}

impl FileServer {
    /// Creates a new FileServer that serves files from the given directory.
    pub fn new<P: Into<PathBuf>>(dir: P) -> Self {
        FileServer { root: dir.into() }
    }

    pub(crate) fn build(self) -> EndpointHandler {
        let root = self.root;

        Arc::new(move |req: &Request, _res: &mut Response| {
            // Static files can only be read, so anything other than GET is treated
            // as not found.
            if req.method() != "GET" {
                return not_found(req);
            }

            let path = match resolve_path(&root, req.path()) {
                Some(path) => path,
                None => return not_found(req),
            };

            match std::fs::read(&path) {
                Ok(data) => FileResponse::new(data, content_type_for(&path)).into(),
                Err(_) => not_found(req),
            }
        })
    }
}

fn not_found(req: &Request) -> Box<dyn crate::responses::HttpResponse> {
    NotFoundError::with_message(format!("Cannot {} {}", req.method(), req.path())).into()
}

/// Resolves a request path to a file inside `root`, rejecting any path that
/// would escape the root directory.
fn resolve_path(root: &Path, req_path: &str) -> Option<PathBuf> {
    let relative = req_path.trim_start_matches('/');

    let mut path = root.to_path_buf();
    for component in Path::new(relative).components() {
        match component {
            // Plain path segments are the only ones allowed.
            Component::Normal(segment) => path.push(segment),
            // "." is harmless and simply ignored.
            Component::CurDir => {}
            // "..", absolute roots and Windows prefixes could escape the root.
            _ => return None,
        }
    }

    // Directories (including the root for "/") serve their index.html.
    if path.is_dir() {
        path.push("index.html");
    }

    Some(path)
}

/// Maps a file extension to a reasonable Content-Type.
fn content_type_for(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("webp") => "image/webp",
        Some("ico") => "image/x-icon",
        Some("pdf") => "application/pdf",
        Some("wasm") => "application/wasm",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}
