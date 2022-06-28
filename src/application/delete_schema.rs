use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{Error, Id, SchemaRepository};

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
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl DeleteSchema {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> DeleteSchema {
        DeleteSchema { schema_repository }
    }

    pub async fn exec(&self, cmd: DeleteSchemaCommand) -> Result<DeleteSchemaResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(schema) = self.schema_repository.find_by_id(&schema_id).await? {
            if schema.configs().len() > 0 {
                return Err(Error::Generic);
            }

            self.schema_repository.delete(&schema_id).await?;

            return Ok(DeleteSchemaResponse {
                schema_id: schema_id.to_string(),
            });
        }

        Err(Error::Generic)
    }
}
