mod application;
mod config;
mod container;
mod domain;
mod handlers;
mod infrastructure;

use axum::{
    routing::{get, post, put},
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
        .route("/schemas", post(handlers::create_schema))
        .route(
            "/schemas/:schema_id",
            get(handlers::get_schema_by_id)
                .put(handlers::update_schema)
                .delete(handlers::delete_schema),
        )
        .route("/schemas/:schema_id/configs", post(handlers::create_config))
        .route(
            "/schemas/:schema_id/configs/:schema_id",
            put(handlers::update_config).delete(handlers::delete_config),
        )
        .route(
            "/schemas/:schema_id/validate",
            post(handlers::validate_config),
        )
        .layer(Extension(container));

    let addr = format!("{}:{}", config.host, config.port);
    println!("Listening on {}", addr);
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
