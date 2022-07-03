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
}

#[derive(Serialize)]
pub struct GetConfigResponse {
    pub schema_id: String,
    pub id: String,
    pub name: String,
    pub data: JsonValue,
    pub valid: bool,
}

pub struct GetConfig {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl GetConfig {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> GetConfig {
        GetConfig { schema_repository }
    }

    pub async fn exec(&self, cmd: GetConfigCommand) -> Result<GetConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let config_id = Id::new(cmd.config_id)?;

            let config = schema.get_config(&config_id)?;

            return Ok(GetConfigResponse {
                schema_id: schema_id.to_string(),
                id: config.id().to_string(),
                name: config.name().to_string(),
                data: config.data().into(),
                valid: config.is_valid(),
            });
        }

        Err(Error::SchemaNotFound(schema_id))
    }
}
