use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{Config, ConfigRepository, Error, Id, Reason, SchemaRepository};

#[derive(Deserialize)]
pub struct CreateConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    pub name: String,
    pub data: JsonValue,
}

#[derive(Serialize)]
pub struct CreateConfigResponse {
    pub schema_id: String,
    pub config_id: String,
    pub valid: bool,
    pub created: bool,
    pub diffs: HashMap<String, Vec<Reason>>,
}

pub struct CreateConfig {
    config_repository: Arc<dyn ConfigRepository + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl CreateConfig {
    pub fn new(
        config_repository: Arc<dyn ConfigRepository + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> CreateConfig {
        CreateConfig {
            config_repository,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: CreateConfigCommand) -> Result<CreateConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let mut config = Config::create(schema.id().clone(), cmd.name, cmd.data.into())?;

            let diff = schema.validate(&mut config);
            let created = if diff.is_empty() {
                self.config_repository.save(&mut config).await?;

                true
            } else {
                false
            };

            return Ok(CreateConfigResponse {
                schema_id: schema.id().to_string(),
                config_id: config.id().to_string(),
                valid: diff.is_empty(),
                created,
                diffs: diff.into_diffs(),
            });
        }

        Err(Error::Generic)
    }
}
