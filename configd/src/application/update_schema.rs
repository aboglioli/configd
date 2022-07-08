use core_lib::events::Publisher;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, Id, SchemaRepository};

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

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let prop = cmd.schema.try_into()?;

            schema.change_root_prop(prop)?;

            self.schema_repository.save(&mut schema).await?;

            self.event_publisher
                .publish(&schema.events())
                .await
                .map_err(Error::Core)?;

            return Ok(UpdateSchemaResponse {
                id: schema.id().to_string(),
            });
        }

        Err(Error::SchemaNotFound(schema_id))
    }
}
