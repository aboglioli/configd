use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{
    application::{
        CreateConfig, CreateConfigCommand, CreateSchema, CreateSchemaCommand, DeleteConfig,
        DeleteConfigCommand, DeleteSchema, DeleteSchemaCommand, GetConfig, GetConfigCommand,
        GetSchema, GetSchemaCommand, UpdateConfig, UpdateConfigCommand, UpdateSchema,
        UpdateSchemaCommand, ValidateConfig, ValidateConfigCommand,
    },
    container::Container,
};

// General
pub async fn health() -> &'static str {
    "OK"
}

// Schema
pub async fn get_schema_by_id(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let serv = GetSchema::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(GetSchemaCommand { schema_id }).await.unwrap();

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

pub async fn update_schema(
    Path(schema_id): Path<String>,
    Json(mut cmd): Json<UpdateSchemaCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    cmd.schema_id = schema_id;

    let serv = UpdateSchema::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await.unwrap();

    (StatusCode::OK, Json(res))
}

pub async fn delete_schema(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let serv = DeleteSchema::new(container.schema_repository.clone());

    let res = serv.exec(DeleteSchemaCommand { schema_id }).await.unwrap();

    (StatusCode::OK, Json(res))
}

// Config
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

pub async fn get_config_by_id(
    Path((schema_id, config_id)): Path<(String, String)>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let serv = GetConfig::new(container.schema_repository.clone());

    let res = serv
        .exec(GetConfigCommand {
            schema_id,
            config_id,
        })
        .await
        .unwrap();

    (StatusCode::OK, Json(res))
}

pub async fn create_config(
    Path(schema_id): Path<String>,
    Json(mut cmd): Json<CreateConfigCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    cmd.schema_id = schema_id;

    let serv = CreateConfig::new(container.schema_repository.clone());

    let res = serv.exec(cmd).await.unwrap();

    (StatusCode::CREATED, Json(res))
}

pub async fn update_config(
    Path((schema_id, config_id)): Path<(String, String)>,
    Json(mut cmd): Json<UpdateConfigCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    cmd.schema_id = schema_id;
    cmd.config_id = config_id;

    let serv = UpdateConfig::new(container.schema_repository.clone());

    let res = serv.exec(cmd).await.unwrap();

    (StatusCode::OK, Json(res))
}

pub async fn delete_config(
    Path((schema_id, config_id)): Path<(String, String)>,
    Extension(container): Extension<Arc<Container>>,
) -> impl IntoResponse {
    let cmd = DeleteConfigCommand {
        schema_id,
        config_id,
    };

    let serv = DeleteConfig::new(container.schema_repository.clone());

    let res = serv.exec(cmd).await.unwrap();

    (StatusCode::OK, Json(res))
}
