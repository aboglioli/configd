use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::{
    application::SchemaDto,
    domain::{Error, Id, PropConverter, SchemaRepository},
};

#[derive(Deserialize)]
pub struct GetSchemaCommand {
    pub id: String,
}

pub type GetSchemaResponse = SchemaDto;

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
        let schema_id = Id::new(cmd.id)?;

        if let Some(schema) = self.schema_repository.find_by_id(&schema_id).await? {
            return Ok(GetSchemaResponse {
                id: schema.id().to_string(),
                name: schema.name().to_string(),
                schema: self.prop_converter.to(schema.into_root_prop())?,
            });
        }

        Err(Error::Generic)
    }
}
