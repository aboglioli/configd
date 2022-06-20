use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{
    application::{
        CreateSchema, CreateSchemaCommand, DeleteSchema, DeleteSchemaCommand, GetSchema,
        GetSchemaCommand, ValidateConfig, ValidateConfigCommand,
    },
    container::Container,
};

pub async fn health() -> &'static str {
    "OK"
}

pub async fn get_schema_by_id(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let serv = GetSchema::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(GetSchemaCommand { id: schema_id }).await.unwrap();

    (StatusCode::OK, Json(res))
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

pub async fn delete_schema(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let serv = DeleteSchema::new(container.schema_repository.clone());

    let res = serv
        .exec(DeleteSchemaCommand { id: schema_id })
        .await
        .unwrap();

    (StatusCode::OK, Json(res))
}

pub async fn validate_config(
    Path(schema_id): Path<String>,
    Json(mut cmd): Json<ValidateConfigCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    cmd.schema_id = schema_id;

    let serv = ValidateConfig::new(container.schema_repository.clone());

    let res = serv.exec(cmd).await.unwrap();

    (StatusCode::OK, Json(res))
}
