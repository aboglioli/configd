use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{Error, Id, Reason, SchemaRepository, Value};

#[derive(Deserialize)]
pub struct ValidateConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    pub data: JsonValue,
}

#[derive(Serialize)]
pub struct ValidateConfigResponse {
    valid: bool,
    diffs: HashMap<String, Vec<Reason>>,
}

pub struct ValidateConfig {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl ValidateConfig {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> ValidateConfig {
        ValidateConfig { schema_repository }
    }

    pub async fn exec(&self, cmd: ValidateConfigCommand) -> Result<ValidateConfigResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let config = Value::from(cmd.data);

            let diff = schema.validate(&config);

            return Ok(ValidateConfigResponse {
                valid: diff.is_empty(),
                diffs: diff.into_diffs(),
            });
        }

        Err(Error::Generic)
    }
}
