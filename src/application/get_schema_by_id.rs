use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::{
    application::SchemaDto,
    domain::{Error, PropConverter, SchemaId, SchemaRepository},
};

#[derive(Deserialize)]
pub struct GetSchemaByIdCommand {
    pub id: String,
}

pub type GetSchemaByIdResponse = SchemaDto;

pub struct GetSchemaById {
    prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl GetSchemaById {
    pub fn new(
        prop_converter: Arc<dyn PropConverter<JsonValue, Error = Error> + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> GetSchemaById {
        GetSchemaById {
            prop_converter,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: GetSchemaByIdCommand) -> Result<GetSchemaByIdResponse, Error> {
        let schema_id = SchemaId::new(cmd.id)?;

        if let Some(schema) = self.schema_repository.find_by_id(&schema_id).await? {
            return Ok(GetSchemaByIdResponse {
                id: schema.id().to_string(),
                name: schema.name().to_string(),
                schema: self.prop_converter.to(schema.into_root_prop())?,
            });
        }

        Err(Error::Generic)
    }
}
