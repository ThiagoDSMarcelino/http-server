# Architecture

An overview of how `http-server` is structured internally and how a request flows through the system.

---

## Module Layout

```text
src/
├── lib.rs                  # Public re-exports
├── headers/
│   ├── headers.rs          # Headers struct (parse, get, set)
│   └── keys.rs             # Standard header name constants
├── request/
│   ├── request.rs          # Request struct and public API
│   ├── request_line.rs     # Parses "METHOD /path HTTP/1.1"
│   ├── request_state.rs    # State machine enum
│   └── body.rs             # Body extraction logic
├── response/
│   ├── response.rs         # Response struct (written to stream)
│   └── status_code.rs      # StatusCode enum
└── server/
    ├── server.rs           # TCP listener, task spawning
    ├── router.rs           # Endpoint registry and dispatch
    ├── handler.rs          # EndpointHandler type alias
    └── responses/
        ├── http_response.rs         # HttpResponse trait
        ├── http_error.rs            # HttpError trait (auto-impl HttpResponse)
        ├── successful/
        │   └── ok_response.rs       # 200 OK
        ├── client_error/
        │   ├── bad_request_error.rs # 400
        │   └── not_found_error.rs   # 404
        └── server_error/
            └── not_implemented_error.rs # 501
```

---

## Request Lifecycle

```text
TCP Connection
      │
      ▼
 Server::serve()
      │
      ├── TcpListener::accept()
      │
      └── tokio::spawn(async move { ... })
                │
                ▼
         Request::from_reader()
                │
                ├── State: Init → RequestLine
                │     Parses: "GET /path?q=1 HTTP/1.1"
                │
                ├── State: RequestLine → Headers
                │     Parses: "Key: Value\r\n" pairs
                │
                ├── State: Headers → Body
                │     Reads N bytes from Content-Length
                │
                └── State: Body → Done
                          │
                          ▼
                    Router::handle()
                          │
                    Matches "METHOD /path"
                          │
                     handler(&req, &mut res)
                          │
                          ▼
                   response.set_result(...)
                   response.set_default_headers()
                   response.write_response(stream)
```

---

## Request Parsing: State Machine

Requests are parsed incrementally from the TCP stream using a 4096-byte read buffer and a state machine with the following states:

| State              | Description                               |
| ------------------ | ----------------------------------------- |
| `StateInit`        | Before any data is read                   |
| `StateRequestLine` | Parsing `METHOD /path HTTP/version`       |
| `StateHeaders`     | Parsing `Key: Value` header lines         |
| `StateBody`        | Reading body bytes up to `Content-Length` |
| `StateDone`        | Request fully parsed                      |

The parser only reads as many bytes as needed and never blocks unnecessarily.

---

## Routing

Routes are stored as a `HashMap<String, EndpointHandler>` keyed by `"METHOD /path"`. For example:

- `router.get("/users", ...)` → key `"GET /users"`
- `router.post("/items", ...)` → key `"POST /items"`

If no matching key is found, the router returns a `NotFoundError` (404) automatically.

The router is consumed by `Server::new()` and compiled into a single `Arc<dyn Fn(...)>` that is cloned per connection.

---

## Response System

Responses are defined through two traits:

### `HttpResponse`

Any type that can produce a raw byte body and report a status code.

```rust
pub trait HttpResponse {
    fn into_response(self: Box<Self>) -> Vec<u8>;
    fn status_code(&self) -> StatusCode;
}
```

### `HttpError`

A simpler trait for error types. Implementing `HttpError` automatically provides an `HttpResponse` implementation that serializes the error to JSON:

```rust
pub trait HttpError: Sync + Send {
    fn message(&self) -> &str;
    fn status_code(&self) -> StatusCode;
}
```

This means adding a new error type requires only implementing `HttpError`, not the full serialization logic.

---

## Concurrency Model

Each accepted TCP connection is handled in its own `tokio::spawn` task. The `EndpointHandler` is wrapped in an `Arc` so it can be cloned cheaply across tasks without copying the underlying function.

All handler types must be `Send + Sync + 'static` to be safely shared across async tasks.
