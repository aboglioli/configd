use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{Error, Id, Reason, SchemaRepository};

#[derive(Deserialize)]
pub struct UpdateConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    #[serde(skip_deserializing)]
    pub config_id: String,
    pub data: JsonValue,
}

#[derive(Serialize)]
pub struct UpdateConfigResponse {
    pub schema_id: String,
    pub config_id: String,
    pub diff: Option<HashMap<String, Vec<Reason>>>,
}

pub struct UpdateConfig {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl UpdateConfig {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> UpdateConfig {
        UpdateConfig { schema_repository }
    }

    pub async fn exec(&self, cmd: UpdateConfigCommand) -> Result<UpdateConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let config_id = Id::new(cmd.config_id)?;

            let diff = schema.update_config(&config_id, cmd.data.into())?;

            self.schema_repository.save(&mut schema).await?;

            return Ok(UpdateConfigResponse {
                schema_id: schema_id.to_string(),
                config_id: config_id.to_string(),
                diff: if !diff.is_empty() {
                    Some(diff.diffs().clone())
                } else {
                    None
                },
            });
        }

        Err(Error::Generic)
    }
}
