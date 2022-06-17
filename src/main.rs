mod domain;

use axum::{routing::get, Router, Server};
use std::net::SocketAddr;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello World" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
