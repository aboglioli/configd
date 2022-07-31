use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{errors::Error, events::Publisher, schemas::SchemaRepository, shared::Id};

#[derive(Deserialize)]
pub struct UpdateSchemaCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    pub schema: JsonValue,
}

#[derive(Serialize)]
pub struct UpdateSchemaResponse {
    pub id: String,
}

pub struct UpdateSchema {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl UpdateSchema {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> UpdateSchema {
        UpdateSchema {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: UpdateSchemaCommand) -> Result<UpdateSchemaResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        let mut schema = self
            .schema_repository
            .find_by_id(&schema_id)
            .await?
            .ok_or_else(|| Error::SchemaNotFound(schema_id.clone()))?;

        schema.change_root_prop(cmd.schema.try_into()?)?;

        self.schema_repository.save(&mut schema).await?;

        self.event_publisher.publish(schema.events()).await?;

        Ok(UpdateSchemaResponse {
            id: schema.id().to_string(),
        })
    }
}
