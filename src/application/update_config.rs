use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{ConfigRepository, Error, Id, Reason, SchemaRepository};

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
    pub valid: bool,
    pub created: bool,
    pub diffs: HashMap<String, Vec<Reason>>,
}

pub struct UpdateConfig {
    config_repository: Arc<dyn ConfigRepository + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl UpdateConfig {
    pub fn new(
        config_repository: Arc<dyn ConfigRepository + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> UpdateConfig {
        UpdateConfig {
            config_repository,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: UpdateConfigCommand) -> Result<UpdateConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;
        let config_id = Id::new(cmd.config_id)?;

        if let Some(mut config) = self
            .config_repository
            .find_by_id(&schema_id, &config_id)
            .await?
        {
            if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
                config.change_data(cmd.data.into())?;

                let diff = schema.validate(&config);

                return Ok(UpdateConfigResponse {
                    schema_id: schema.id().to_string(),
                    config_id: config.id().to_string(),
                    valid: diff.is_empty(),
                    created: if diff.is_empty() {
                        self.config_repository.save(&mut config).await?;
                        self.schema_repository.save(&mut schema).await?;

                        true
                    } else {
                        false
                    },
                    diffs: diff.into_diffs(),
                });
            }
        }

        Err(Error::Generic)
    }
}
