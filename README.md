# routeum

Simple proc macro that brings [Actix Web](https://actix.rs/) style function attributes to [axum](https://github.com/tokio-rs/axum).

- This currently requires `crate::State` to exist; This is the State used by your axum router

## Example

```rust
use axum::{Router, extract::Path, response::IntoResponse};

#[get("/")]
async fn index() -> impl IntoResponse {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {name}!")
}

#[tokio::main]
async fn main() {
    let mut router = Router::new();
    router = index(router);
    router = hello(router);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
```
