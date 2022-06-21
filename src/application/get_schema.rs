use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, Id, PropConverter, SchemaRepository};

#[derive(Deserialize)]
pub struct GetSchemaCommand {
    pub schema_id: String,
}

#[derive(Serialize)]
pub struct GetSchemaResponse {
    pub id: String,
    pub name: String,
    pub schema: JsonValue,
    pub configs: usize,
}

pub struct GetSchema {
    prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl GetSchema {
    pub fn new(
        prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> GetSchema {
        GetSchema {
            prop_converter,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: GetSchemaCommand) -> Result<GetSchemaResponse, Error> {
        let schema_id = Id::new(cmd.schema_id)?;

        if let Some(schema) = self.schema_repository.find_by_id(&schema_id).await? {
            return Ok(GetSchemaResponse {
                id: schema.id().to_string(),
                name: schema.name().to_string(),
                schema: self.prop_converter.to(schema.root_prop().clone())?,
                configs: schema.configs().len(),
            });
        }

        Err(Error::Generic)
    }
}
