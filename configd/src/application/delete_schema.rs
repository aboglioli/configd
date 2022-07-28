use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{errors::Error, events::Publisher, schemas::SchemaRepository, shared::Id};

#[derive(Deserialize)]
pub struct DeleteSchemaCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
}

#[derive(Serialize)]
pub struct DeleteSchemaResponse {
    pub schema_id: String,
}

pub struct DeleteSchema {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl DeleteSchema {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> DeleteSchema {
        DeleteSchema {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: DeleteSchemaCommand) -> Result<DeleteSchemaResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            schema.delete()?;

            self.schema_repository.save(&mut schema).await?;

            self.event_publisher.publish(&schema.events()).await?;

            return Ok(DeleteSchemaResponse {
                schema_id: schema_id.to_string(),
            });
        }

        Err(Error::SchemaNotFound(schema_id))
    }
}
