use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

use crate::{
    application::{CreateSchema, CreateSchemaCommand},
    container::Container,
};

pub async fn health() -> &'static str {
    "OK"
}

#[axum_macros::debug_handler]
pub async fn create_schema(
    Json(cmd): Json<CreateSchemaCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let serv = CreateSchema::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await.unwrap();

    (StatusCode::CREATED, Json(res))
}
