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
    #[serde(skip_deserializing)]
    pub instance: Option<String>,
}

#[derive(Serialize)]
pub struct ConfigAccessDto {
    pub source: String,
    pub instance: String,
    pub timestamp: DateTime<Utc>,
    pub previous: Option<DateTime<Utc>>,
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
            let source = cmd.source.map(Id::new).transpose()?;
            let instance = cmd.instance.map(Id::new).transpose()?;

            let config = schema.get_config(&config_id, source, instance)?;
            let data = schema.populate_config_data(&config)?;
            let checksum = data.checksum();

            let res = GetConfigResponse {
                schema_id: schema_id.to_string(),
                id: config.id().to_string(),
                name: config.name().to_string(),
                data: data.into(),
                valid: config.is_valid(),
                checksum,
                accesses: config
                    .accesses()
                    .iter()
                    .map(|access| ConfigAccessDto {
                        source: access.source().to_string(),
                        instance: access.instance().to_string(),
                        timestamp: *access.timestamp(),
                        previous: access.previous().copied(),
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
