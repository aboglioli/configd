use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

use crate::domain::{errors::Error, schemas::SchemaRepository, shared::Id, values::Reason};

#[derive(Deserialize)]
pub struct ValidateConfigCommand {
    #[serde(skip_deserializing)]
    pub schema_id: String,
    pub data: JsonValue,
}

#[derive(Serialize)]
pub struct ValidateConfigResponse {
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

        let schema = self
            .schema_repository
            .find_by_id(&schema_id)
            .await?
            .ok_or_else(|| Error::SchemaNotFound(schema_id.clone()))?;

        let diff = schema.root_prop().validate(&cmd.data.into());

        Ok(ValidateConfigResponse {
            diffs: diff.diffs().clone(),
        })
    }
}
