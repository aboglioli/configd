use core_lib::events::Publisher;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, Id, SchemaRepository};

#[derive(Deserialize)]
pub struct CreateConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    pub name: String,
    pub data: JsonValue,
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

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let config_id = Id::slug(&cmd.name)?;

            schema.add_config(config_id.clone(), cmd.name, cmd.data.into())?;

            self.schema_repository.save(&mut schema).await?;

            self.event_publisher
                .publish(&schema.events())
                .await
                .map_err(Error::CouldNotPublishEvents)?;

            return Ok(CreateConfigResponse {
                schema_id: schema_id.to_string(),
                config_id: config_id.to_string(),
            });
        }

        Err(Error::SchemaNotFound(schema_id))
    }
}
