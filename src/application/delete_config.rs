use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{Error, Id, SchemaRepository};

#[derive(Deserialize)]
pub struct DeleteConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    #[serde(skip_deserializing)]
    pub config_id: String,
}

#[derive(Serialize)]
pub struct DeleteConfigResponse {
    pub schema_id: String,
    pub config_id: String,
}

pub struct DeleteConfig {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl DeleteConfig {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> DeleteConfig {
        DeleteConfig { schema_repository }
    }

    pub async fn exec(&self, cmd: DeleteConfigCommand) -> Result<DeleteConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let config_id = Id::new(cmd.config_id)?;

            schema.delete_config(&config_id)?;

            self.schema_repository.save(&mut schema).await?;

            return Ok(DeleteConfigResponse {
                schema_id: schema_id.to_string(),
                config_id: config_id.to_string(),
            });
        }

        Err(Error::Generic)
    }
}
