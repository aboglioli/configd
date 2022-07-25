use core_lib::events::Publisher;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{Error, Id, Password, SchemaRepository};

#[derive(Deserialize)]
pub struct DeleteConfigPasswordCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    #[serde(skip_deserializing)]
    pub config_id: String,
    #[serde(skip_deserializing)]
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct DeleteConfigPasswordResponse {
    pub schema_id: String,
    pub config_id: String,
}

pub struct DeleteConfigPassword {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl DeleteConfigPassword {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> DeleteConfigPassword {
        DeleteConfigPassword {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(
        &self,
        cmd: DeleteConfigPasswordCommand,
    ) -> Result<DeleteConfigPasswordResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let config_id = Id::new(cmd.config_id)?;
            let password = cmd.password.map(Password::new).transpose()?;

            schema.delete_config_password(&config_id, password.as_ref())?;

            self.schema_repository.save(&mut schema).await?;

            self.event_publisher
                .publish(&schema.events())
                .await
                .map_err(Error::Core)?;

            return Ok(DeleteConfigPasswordResponse {
                schema_id: schema_id.to_string(),
                config_id: config_id.to_string(),
            });
        }

        Err(Error::SchemaNotFound(schema_id))
    }
}
