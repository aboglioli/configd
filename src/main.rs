mod application;
mod config;
mod container;
mod domain;
mod handlers;
mod infrastructure;

use axum::{
    routing::{get, post},
    Extension, Router, Server,
};
use std::sync::Arc;

use crate::{config::Config, container::Container};

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();

    let container = Arc::new(Container::build(&config).unwrap());

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/schema", post(handlers::create_schema))
        .route("/schema/:schema_id", get(handlers::get_schema_by_id))
        .layer(Extension(container));

    let addr = format!("{}:{}", config.host, config.port);
    println!("Listening on {}", addr);
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
