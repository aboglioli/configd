use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::{
    configs::Password, errors::Error, events::Publisher, schemas::SchemaRepository, shared::Id,
};

#[derive(Deserialize)]
pub struct ChangeConfigPasswordCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    #[serde(skip_deserializing)]
    pub config_id: String,
    #[serde(skip_deserializing)]
    pub old_password: Option<String>,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ChangeConfigPasswordResponse {
    pub schema_id: String,
    pub config_id: String,
}

pub struct ChangeConfigPassword {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl ChangeConfigPassword {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> ChangeConfigPassword {
        ChangeConfigPassword {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(
        &self,
        cmd: ChangeConfigPasswordCommand,
    ) -> Result<ChangeConfigPasswordResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        let mut schema = self
            .schema_repository
            .find_by_id(&schema_id)
            .await?
            .ok_or_else(|| Error::SchemaNotFound(schema_id.clone()))?;

        let config_id = Id::new(cmd.config_id)?;
        let old_password = cmd.old_password.map(Password::new).transpose()?;
        let new_password = Password::new(cmd.new_password)?;

        schema.change_config_password(&config_id, old_password.as_ref(), new_password)?;

        self.schema_repository.save(&mut schema).await?;

        self.event_publisher.publish(schema.events()).await?;

        Ok(ChangeConfigPasswordResponse {
            schema_id: schema_id.to_string(),
            config_id: config_id.to_string(),
        })
    }
}
