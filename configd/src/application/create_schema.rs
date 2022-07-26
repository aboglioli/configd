use core_lib::events::Publisher;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{
    errors::Error,
    schemas::{Schema, SchemaRepository},
    shared::Id,
};

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
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl CreateSchema {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> CreateSchema {
        CreateSchema {
            event_publisher,
            schema_repository,
        }
    }

    pub async fn exec(&self, cmd: CreateSchemaCommand) -> Result<CreateSchemaResponse, Error> {
        let id = Id::slug(&cmd.name)?;

        if self.schema_repository.exists(&id).await? {
            return Err(Error::SchemaAlreadyExists(id));
        }

        let prop = cmd.schema.try_into()?;

        let mut schema = Schema::create(id, cmd.name, prop)?;

        self.schema_repository.save(&mut schema).await?;

        self.event_publisher
            .publish(&schema.events())
            .await
            .map_err(Error::Core)?;

        Ok(CreateSchemaResponse {
            id: schema.id().to_string(),
        })
    }
}
