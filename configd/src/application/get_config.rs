use chrono::{DateTime, Utc};
use core_lib::events::Publisher;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, Id, SchemaRepository};

#[derive(Deserialize)]
pub struct GetConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    #[serde(skip_deserializing)]
    pub config_id: String,
    #[serde(skip_deserializing)]
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct ConfigAccessDto {
    pub source: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct GetConfigResponse {
    pub schema_id: String,
    pub id: String,
    pub name: String,
    pub data: JsonValue,
    pub valid: bool,
    pub checksum: String,
    pub accesses: Vec<ConfigAccessDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

pub struct GetConfig {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl GetConfig {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> GetConfig {
        GetConfig {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: GetConfigCommand) -> Result<GetConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let config_id = Id::new(cmd.config_id)?;

            let config = schema.get_config(&config_id, cmd.source)?;

            let res = GetConfigResponse {
                schema_id: schema_id.to_string(),
                id: config.id().to_string(),
                name: config.name().to_string(),
                data: config.data().into(),
                valid: config.is_valid(),
                checksum: config.checksum().to_string(),
                accesses: config
                    .accesses()
                    .iter()
                    .map(|access| ConfigAccessDto {
                        source: access.source().to_string(),
                        timestamp: *access.timestamp(),
                    })
                    .collect(),
                created_at: *config.timestamps().created_at(),
                updated_at: *config.timestamps().updated_at(),
                version: config.version().value(),
            };

            self.event_publisher
                .publish(&schema.events())
                .await
                .map_err(Error::Core)?;

            self.schema_repository.save(&mut schema).await?;

            return Ok(res);
        }

        Err(Error::SchemaNotFound(schema_id))
    }
}
