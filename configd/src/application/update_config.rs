use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{
    configs::Password, errors::Error, events::Publisher, schemas::SchemaRepository, shared::Id,
};

#[derive(Deserialize)]
pub struct UpdateConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    #[serde(skip_deserializing)]
    pub config_id: String,
    pub data: JsonValue,
    #[serde(skip_deserializing)]
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateConfigResponse {
    pub schema_id: String,
    pub config_id: String,
}

pub struct UpdateConfig {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl UpdateConfig {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> UpdateConfig {
        UpdateConfig {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: UpdateConfigCommand) -> Result<UpdateConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        let mut schema = self
            .schema_repository
            .find_by_id(&schema_id)
            .await?
            .ok_or_else(|| Error::SchemaNotFound(schema_id.clone()))?;

        let config_id = Id::new(cmd.config_id)?;
        let password = cmd.password.map(Password::new).transpose()?;

        schema.update_config(&config_id, cmd.data.into(), password.as_ref())?;

        self.schema_repository.save(&mut schema).await?;

        self.event_publisher.publish(schema.events()).await?;

        Ok(UpdateConfigResponse {
            schema_id: schema_id.to_string(),
            config_id: config_id.to_string(),
        })
    }
}
