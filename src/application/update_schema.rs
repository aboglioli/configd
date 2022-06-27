use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, Id, PropConverter, SchemaRepository};

#[derive(Deserialize)]
pub struct UpdateSchemaCommand {
    pub schema_id: String,
    pub schema: JsonValue,
}

#[derive(Serialize)]
pub struct UpdateSchemaResponse {
    pub id: String,
}

pub struct UpdateSchema {
    prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl UpdateSchema {
    pub fn new(
        prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> UpdateSchema {
        UpdateSchema {
            prop_converter,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: UpdateSchemaCommand) -> Result<UpdateSchemaResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(mut schema) = self.schema_repository.find_by_id(&schema_id).await? {
            let prop = self.prop_converter.from(cmd.schema)?;

            // TODO: update related configs
            schema.change_root_prop(prop)?;

            self.schema_repository.save(&mut schema).await?;

            return Ok(UpdateSchemaResponse {
                id: schema.id().to_string(),
            });
        }

        Err(Error::Generic)
    }
}
