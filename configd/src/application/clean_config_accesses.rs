use async_trait::async_trait;
use core_lib::{
    errors::Result,
    events::{Event, Handler},
};
use std::sync::Arc;

use crate::domain::{
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
    async fn handle(&self, event: &Event) -> Result<()> {
        if event.topic() == "config.accessed" {
            let payload: ConfigAccessed = event.deserialize_payload()?;

            let schema_id = Id::new(payload.schema_id).unwrap();
            let mut schema = self
                .schema_repository
                .find_by_id(&schema_id)
                .await
                .unwrap()
                .unwrap();

            let config_id = Id::new(payload.id).unwrap();

            schema.clean_config_accesses(&config_id).unwrap();

            self.schema_repository.save(&mut schema).await.unwrap();
        }

        Ok(())
    }
}
