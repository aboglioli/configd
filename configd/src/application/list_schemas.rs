use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::domain::{Error, SchemaRepository};

#[derive(Deserialize)]
pub struct ListSchemasCommand {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Serialize)]
pub struct SchemaConfigDto {
    pub id: String,
    pub name: String,
    pub valid: bool,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

#[derive(Serialize)]
pub struct SchemaDto {
    pub id: String,
    pub name: String,
    pub schema: JsonValue,
    pub configs: Vec<SchemaConfigDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

#[derive(Serialize)]
pub struct ListSchemasResponse {
    pub offset: u64,
    pub limit: u64,
    pub total: u64,
    pub data: Vec<SchemaDto>,
}

pub struct ListSchemas {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl ListSchemas {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> ListSchemas {
        ListSchemas { schema_repository }
    }

    pub async fn exec(&self, cmd: ListSchemasCommand) -> Result<ListSchemasResponse, Error> {
        let schemas_page = self.schema_repository.find(cmd.offset, cmd.limit).await?;

        Ok(ListSchemasResponse {
            offset: schemas_page.offset(),
            limit: schemas_page.limit(),
            total: schemas_page.total(),
            data: schemas_page
                .into_data()
                .into_iter()
                .map(|schema| {
                    Ok(SchemaDto {
                        id: schema.id().to_string(),
                        name: schema.name().to_string(),
                        schema: schema.root_prop().clone().try_into()?,
                        configs: schema
                            .configs()
                            .values()
                            .map(|config| SchemaConfigDto {
                                id: config.id().to_string(),
                                name: config.name().to_string(),
                                valid: config.is_valid(),
                                checksum: hex::encode(config.checksum()),
                                created_at: *config.timestamps().created_at(),
                                updated_at: *config.timestamps().updated_at(),
                                version: config.version().value(),
                            })
                            .collect(),
                        created_at: *schema.timestamps().created_at(),
                        updated_at: *schema.timestamps().updated_at(),
                        version: schema.version().value(),
                    })
                })
                .collect::<Result<Vec<SchemaDto>, Error>>()?,
        })
    }
}
