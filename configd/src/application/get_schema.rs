use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, Id, SchemaRepository};

#[derive(Deserialize)]
pub struct GetSchemaCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
}

#[derive(Serialize)]
pub struct ConfigDto {
    pub id: String,
    pub name: String,
    pub valid: bool,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

#[derive(Serialize)]
pub struct GetSchemaResponse {
    pub id: String,
    pub name: String,
    pub schema: JsonValue,
    pub configs: Vec<ConfigDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

pub struct GetSchema {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl GetSchema {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> GetSchema {
        GetSchema { schema_repository }
    }

    pub async fn exec(&self, cmd: GetSchemaCommand) -> Result<GetSchemaResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(schema) = self.schema_repository.find_by_id(&schema_id).await? {
            return Ok(GetSchemaResponse {
                id: schema.id().to_string(),
                name: schema.name().to_string(),
                schema: schema.root_prop().clone().try_into()?,
                configs: schema
                    .configs()
                    .values()
                    .map(|config| ConfigDto {
                        id: config.id().to_string(),
                        name: config.name().to_string(),
                        valid: config.is_valid(),
                        checksum: hex::encode(config.checksum()),
                        created_at: config.timestamps().created_at().clone(),
                        updated_at: config.timestamps().updated_at().clone(),
                        version: config.version().value(),
                    })
                    .collect(),
                created_at: schema.timestamps().created_at().clone(),
                updated_at: schema.timestamps().updated_at().clone(),
                version: schema.version().value(),
            });
        }

        Err(Error::SchemaNotFound(schema_id))
    }
}
