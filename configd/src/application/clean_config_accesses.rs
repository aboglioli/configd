use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{
    errors::Error,
    events::{Event, Handler},
    schemas::{ConfigAccessed, SchemaRepository},
    shared::Id,
};

pub struct CleanConfigAccesses {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl CleanConfigAccesses {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> CleanConfigAccesses {
        CleanConfigAccesses { schema_repository }
    }
}

#[async_trait]
impl Handler for CleanConfigAccesses {
    async fn handle(&self, event: &Event) -> Result<(), Error> {
        if event.topic() == "config.accessed" {
            let payload: ConfigAccessed = event.deserialize_payload()?;

            let schema_id = Id::new(payload.schema_id)?;
            let mut schema = self
                .schema_repository
                .find_by_id(&schema_id)
                .await?
                .ok_or_else(|| Error::SchemaNotFound(schema_id.clone()))?;

            let config_id = Id::new(payload.id)?;

            schema.clean_config_accesses(&config_id)?;

            self.schema_repository.save(&mut schema).await?;
        }

        Ok(())
    }
}
