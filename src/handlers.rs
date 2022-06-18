use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{
    application::{CreateSchema, CreateSchemaCommand, GetSchemaById, GetSchemaByIdCommand},
    container::Container,
};

pub async fn health() -> &'static str {
    "OK"
}

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

pub async fn get_schema_by_id(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let serv = GetSchemaById::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv
        .exec(GetSchemaByIdCommand { id: schema_id })
        .await
        .unwrap();

    (StatusCode::OK, Json(res))
}
