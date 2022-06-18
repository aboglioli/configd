use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, PropConverter, Schema, SchemaRepository};

#[derive(Deserialize)]
pub struct CreateSchemaCommand {
    pub name: String,
    pub schema: JsonValue,
}

#[derive(Serialize)]
pub struct CreateSchemaResponse {
    pub id: String,
}

pub struct CreateSchema {
    prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl CreateSchema {
    pub fn new(
        prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> CreateSchema {
        CreateSchema {
            prop_converter,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: CreateSchemaCommand) -> Result<CreateSchemaResponse, Error> {
        let prop = self.prop_converter.from(cmd.schema)?;

        let mut schema = Schema::create(cmd.name, prop)?;

        self.schema_repository.save(&mut schema).await?;

        Ok(CreateSchemaResponse {
            id: schema.id().to_string(),
        })
    }
}
