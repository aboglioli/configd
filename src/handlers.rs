use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::Arc;

use crate::{
    application::{
        CreateConfig, CreateConfigCommand, CreateSchema, CreateSchemaCommand, DeleteConfig,
        DeleteConfigCommand, DeleteSchema, DeleteSchemaCommand, GetConfig, GetConfigCommand,
        GetSchema, GetSchemaCommand, UpdateConfig, UpdateConfigCommand, UpdateSchema,
        UpdateSchemaCommand, ValidateConfig, ValidateConfigCommand,
    },
    container::Container,
    domain::Error,
};

// Error
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // let (status, code) = match {
        //
        // }

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

        let body = Json(json!({
            "code": self.code().to_string(),
            "message": self.to_string(),
            "diffs": if let Error::InvalidConfig(diff) = self {
                Some(diff.diffs().clone())
            } else {
                None
            },
        }));

        (status, body).into_response()
    }
}

// General
pub async fn health() -> &'static str {
    "OK"
}

// Schema
pub async fn get_schema_by_id(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = GetSchema::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(GetSchemaCommand { schema_id }).await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn create_schema(
    Json(cmd): Json<CreateSchemaCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = CreateSchema::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::CREATED, Json(res)))
}

#[axum_macros::debug_handler]
pub async fn update_schema(
    Path(schema_id): Path<String>,
    Json(mut cmd): Json<UpdateSchemaCommand>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    cmd.schema_id = schema_id;

    let serv = UpdateSchema::new(
        container.prop_converter.clone(),
        container.schema_repository.clone(),
    );

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::OK, Json(res)))
}

pub async fn delete_schema(
    Path(schema_id): Path<String>,
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = DeleteSchema::new(container.schema_repository.clone());

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
    Extension(container): Extension<Arc<Container>>,
) -> Result<impl IntoResponse, Error> {
    let serv = GetConfig::new(container.schema_repository.clone());

    let res = serv
        .exec(GetConfigCommand {
            schema_id,
            config_id,
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

    let serv = CreateConfig::new(container.schema_repository.clone());

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

    let serv = UpdateConfig::new(container.schema_repository.clone());

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

    let serv = DeleteConfig::new(container.schema_repository.clone());

    let res = serv.exec(cmd).await?;

    Ok((StatusCode::OK, Json(res)))
}
