use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{Error, SchemaId, SchemaRepository};

#[derive(Deserialize)]
pub struct DeleteSchemaCommand {
    pub id: String,
}

#[derive(Serialize)]
pub struct DeleteSchemaResponse {
    pub id: String,
}

pub struct DeleteSchema {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl DeleteSchema {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> DeleteSchema {
        DeleteSchema { schema_repository }
    }

    pub async fn exec(&self, cmd: DeleteSchemaCommand) -> Result<DeleteSchemaResponse, Error> {
        let schema_id = SchemaId::new(cmd.id)?;

        self.schema_repository.delete(&schema_id).await?;

        Err(Error::Generic)
    }
}
