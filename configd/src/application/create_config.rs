use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{
    configs::Password, errors::Error, events::Publisher, schemas::SchemaRepository, shared::Id,
};

#[derive(Deserialize)]
pub struct CreateConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    pub name: String,
    pub data: JsonValue,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct CreateConfigResponse {
    pub schema_id: String,
    pub config_id: String,
}

pub struct CreateConfig {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl CreateConfig {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> CreateConfig {
        CreateConfig {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: CreateConfigCommand) -> Result<CreateConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        let mut schema = self
            .schema_repository
            .find_by_id(&schema_id)
            .await?
            .ok_or_else(|| Error::SchemaNotFound(schema_id.clone()))?;

        let config_id = Id::slug(&cmd.name)?;
        let password = cmd.password.map(Password::new).transpose()?;

        schema.add_config(config_id.clone(), cmd.name, cmd.data.into(), password)?;

        self.schema_repository.save(&mut schema).await?;

        self.event_publisher.publish(schema.events()).await?;

        Ok(CreateConfigResponse {
            schema_id: schema_id.to_string(),
            config_id: config_id.to_string(),
        })
    }
}
