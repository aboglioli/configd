use axum::{
    extract::{Extension, Json, Path, Query},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

use crate::{
    application::{
        CreateConfig, CreateConfigCommand, CreateSchema, CreateSchemaCommand, DeleteConfig,
        DeleteConfigCommand, DeleteSchema, DeleteSchemaCommand, GetConfig, GetConfigCommand,
        GetSchema, GetSchemaCommand, ListSchemas, ListSchemasCommand, UpdateConfig,
        UpdateConfigCommand, UpdateSchema, UpdateSchemaCommand, ValidateConfig,
        ValidateConfigCommand,
    },
    container::Container,
    domain::{Error, Reason},
};

// Error
#[derive(Serialize)]
pub struct ErrorDto {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffs: Option<HashMap<String, Vec<Reason>>>,
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::SchemaNotFound(_) | Error::ConfigNotFound(_) => StatusCode::NOT_FOUND,
            Error::EmptyId
            | Error::EmptyName
            | Error::EmptyInterval
            | Error::MismatchedKinds { .. }
            | Error::InvalidArray
            | Error::UnknownRootProp
            | Error::SchemaAlreadyExists(_)
            | Error::SchemaContainsConfigs(_)
            | Error::ConfigAlreadyExists(_)
            | Error::InvalidConfig(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErrorDto {
            code: self.code().to_string(),
            message: self.to_string(),
            diffs: if let Error::InvalidConfig(diff) = self {
                Some(diff.diffs().clone())
            } else {
                None
            },
        };

        (status, Json(body)).into_response()
    }
}

// General
pub async fn health() -> &'static str {
    "OK"
}

// Schema
pub async fn list_schemas(
    Query(cmd): Query<ListSchemasCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = ListSchemas::new(container.schema_repository.clone());

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn get_schema_by_id(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = GetSchema::new(container.schema_repository.clone());

    let res = serv.exec(GetSchemaCommand { schema_id }).await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn create_schema(
    Json(cmd): Json<CreateSchemaCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = CreateSchema::new(
        container.event_publisher.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::CREATED, Json(res)))
}

pub async fn update_schema(
    Path(schema_id): Path<String>,
    Json(mut cmd): Json<UpdateSchemaCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    cmd.schema_id = schema_id;

    let serv = UpdateSchema::new(
        container.event_publisher.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn delete_schema(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = DeleteSchema::new(
        container.event_publisher.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(DeleteSchemaCommand { schema_id }).await?;

    Ok((StatusCode::OK, Json(res)))
}

// Config
pub async fn validate_config(
    Path(schema_id): Path<String>,
    Json(mut cmd): Json<ValidateConfigCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    cmd.schema_id = schema_id;

    let serv = ValidateConfig::new(container.schema_repository.clone());

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn get_config_by_id(
    Path((schema_id, config_id)): Path<(String, String)>,
    headers: header::HeaderMap,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = GetConfig::new(
        container.event_publisher.clone(),
        container.schema_repository.clone(),
    );

    let res = serv
        .exec(GetConfigCommand {
            schema_id,
            config_id,
            source: headers
                .get("X-Configd-Source")
                .map(|header| header.to_str())
                .transpose()
                .unwrap_or(None)
                .map(|header| header.to_string()),
            instance: headers
                .get("X-Configd-Instance")
                .map(|header| header.to_str())
                .transpose()
                .unwrap_or(None)
                .map(|header| header.to_string()),
        })
        .await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn create_config(
    Path(schema_id): Path<String>,
    Json(mut cmd): Json<CreateConfigCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    cmd.schema_id = schema_id;

    let serv = CreateConfig::new(
        container.event_publisher.clone(),
        container.hasher.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::CREATED, Json(res)))
}

pub async fn update_config(
    Path((schema_id, config_id)): Path<(String, String)>,
    Json(mut cmd): Json<UpdateConfigCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    cmd.schema_id = schema_id;
    cmd.config_id = config_id;

    let serv = UpdateConfig::new(
        container.event_publisher.clone(),
        container.hasher.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn delete_config(
    Path((schema_id, config_id)): Path<(String, String)>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let cmd = DeleteConfigCommand {
        schema_id,
        config_id,
    };

    let serv = DeleteConfig::new(
        container.event_publisher.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::OK, Json(res)))
}
