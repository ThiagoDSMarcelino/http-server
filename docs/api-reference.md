# API Reference

Complete reference for all public types and functions in the `http-server` library.

---

## `Server`

The main entry point. Binds to a TCP address and dispatches incoming connections to the router.

```rust
pub struct Server { ... }
```

### Server Methods

#### `Server::new(addr: &str, router: Router) -> Self`

Creates a new server bound to the given address.

```rust
let server = Server::new("127.0.0.1:8080", router);
```

#### `async Server::serve(self) -> Result<(), std::io::Error>`

Starts listening for connections. Each connection is handled in a separate Tokio task. Runs indefinitely until an I/O error occurs.

```rust
server.serve().await?;
```

---

## `Router`

Registers handlers for specific HTTP methods and paths.

```rust
pub struct Router { ... }
```

### Router Methods

#### `Router::new() -> Self`

Creates an empty router.

#### `router.get(path: &str, handler: EndpointHandler)`

#### `router.post(path: &str, handler: EndpointHandler)`

#### `router.put(path: &str, handler: EndpointHandler)`

#### `router.delete(path: &str, handler: EndpointHandler)`

#### `router.patch(path: &str, handler: EndpointHandler)`

Registers a handler for the given method and path. Unmatched routes automatically return a `404 Not Found` response.

```rust
router.get("/users", Arc::new(|req, res| {
    OkResponse::from(vec!["alice", "bob"]).into()
}));
```

---

## `EndpointHandler`

Type alias for route handlers.

```rust
pub type EndpointHandler = Arc<dyn Fn(&Request, &mut Response) -> Box<dyn HttpResponse> + Send + Sync + 'static>;
```

Handlers receive a reference to the parsed `Request` and a mutable reference to the `Response` (for setting headers), and must return a boxed `HttpResponse`.

---

## `Request`

Represents a fully parsed HTTP request.

```rust
pub struct Request { ... }
```

### Request Methods

| Method           | Return type                | Description                       |
| ---------------- | -------------------------- | --------------------------------- |
| `method()`       | `&str`                     | HTTP method (`GET`, `POST`, etc.) |
| `path()`         | `&str`                     | Request path (e.g. `/users/42`)   |
| `http_version()` | `&str`                     | HTTP version (e.g. `HTTP/1.1`)    |
| `query()`        | `&HashMap<String, String>` | Parsed query string parameters    |
| `headers()`      | `&Headers`                 | Request headers                   |
| `body()`         | `&Vec<u8>`                 | Raw request body bytes            |

### Example

```rust
router.get("/search", Arc::new(|req, _| {
    let term = req.query().get("q").cloned().unwrap_or_default();
    OkResponse::from(format!("Searching for: {}", term)).into()
}));
```

### Supported HTTP Methods

`GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `OPTIONS`, `PATCH`, `TRACE`, `CONNECT`

### Supported HTTP Versions

`HTTP/1.0`, `HTTP/1.1`, `HTTP/2.0`

---

## `Response`

Represents the outgoing HTTP response. Passed mutably to handlers so headers can be customized before the response is written.

```rust
pub struct Response { ... }
```

### Response Methods

| Method          | Return type    | Description                     |
| --------------- | -------------- | ------------------------------- |
| `headers_mut()` | `&mut Headers` | Access headers for modification |

### Default Headers

The server automatically sets the following headers on every response:

| Header           | Value                             |
| ---------------- | --------------------------------- |
| `Content-Length` | Byte length of the response body  |
| `Content-Type`   | `application/json; charset=utf-8` |
| `Connection`     | `close`                           |

---

## `StatusCode`

Enum of supported HTTP status codes.

```rust
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
}
```

### `status_code.as_str() -> &'static str`

Returns the standard reason phrase (e.g. `"OK"`, `"Not Found"`).

---

## `Headers`

Case-insensitive HTTP header storage. Available on both `Request` (read-only) and `Response` (mutable).

```rust
pub struct Headers { ... }
```

### Headers Methods

| Method                             | Description                                |
| ---------------------------------- | ------------------------------------------ |
| `get::<T>(key: &str) -> Option<T>` | Get and parse a header value               |
| `set(key: &str, value: &str)`      | Set a header, replacing any existing value |
| `add(key: &str, value: &str)`      | Append to a header (comma-separated)       |
| `contains(key: &str) -> bool`      | Check if a header exists                   |
| `iter()`                           | Iterate over all `(key, value)` pairs      |

### Common Header Key Constants

Available under `http_server::headers::keys`:

```rust
pub const CONTENT_LENGTH_HEADER: &str = "Content-Length";
pub const CONTENT_TYPE_KEY: &str = "Content-Type";
pub const CONNECTION_HEADER: &str = "Connection";
```

---

## Response Types

All response types implement `HttpResponse` and can be returned from handlers using `.into()`.

### `HttpResponse` Trait

```rust
pub trait HttpResponse {
    fn into_response(self: Box<Self>) -> Vec<u8>;
    fn status_code(&self) -> StatusCode;
}
```

---

### `OkResponse` — 200 OK

```rust
pub struct OkResponse { ... }
```

| Constructor                 | Description                             |
| --------------------------- | --------------------------------------- |
| `OkResponse::new()`         | Empty 200 response                      |
| `OkResponse::from(data: T)` | Serializes `T` to JSON (`T: Serialize`) |

```rust
OkResponse::from(json!({ "id": 1, "name": "Alice" })).into()
```

---

### `BadRequestError` — 400 Bad Request

```rust
pub struct BadRequestError { ... }
```

| Constructor                          | Description     |
| ------------------------------------ | --------------- |
| `BadRequestError::new()`             | Default message |
| `BadRequestError::with_message(msg)` | Custom message  |

---

### `NotFoundError` — 404 Not Found

```rust
pub struct NotFoundError { ... }
```

| Constructor                        | Description     |
| ---------------------------------- | --------------- |
| `NotFoundError::new()`             | Default message |
| `NotFoundError::with_message(msg)` | Custom message  |

---

### `NotImplementedError` — 501 Not Implemented

```rust
pub struct NotImplementedError { ... }
```

| Constructor                              | Description     |
| ---------------------------------------- | --------------- |
| `NotImplementedError::new()`             | Default message |
| `NotImplementedError::with_message(msg)` | Custom message  |

---

### Error JSON Format

All error types (implementing `HttpError`) are serialized to a consistent JSON structure:

```json
{
  "error": "The resource you are looking for was not found.",
  "message": "Not Found",
  "status_code": 404
}
```
