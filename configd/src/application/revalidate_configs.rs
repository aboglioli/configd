use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{
    errors::Error,
    events::{Event, Handler, Publisher},
    schemas::{SchemaRepository, SchemaRootPropChanged},
    shared::Id,
};

pub struct RevalidateConfigs {
    event_publisher: Arc<dyn Publisher + Sync + Send>,
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl RevalidateConfigs {
    pub fn new(
        event_publisher: Arc<dyn Publisher + Sync + Send>,
        schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
    ) -> RevalidateConfigs {
        RevalidateConfigs {
            event_publisher,
            schema_repository,
        }
    }
}

#[async_trait]
impl Handler for RevalidateConfigs {
    async fn handle(&self, event: &Event) -> Result<(), Error> {
        if event.topic() == "schema.root_prop_changed" {
            let payload: SchemaRootPropChanged = event.deserialize_payload()?;

            let schema_id = Id::new(payload.id).unwrap();

            let mut schema = self
                .schema_repository
                .find_by_id(&schema_id)
                .await?
                .ok_or_else(|| Error::SchemaNotFound(schema_id.clone()))?;

            schema.revalidate_configs()?;

            self.schema_repository.save(&mut schema).await.unwrap();

            self.event_publisher.publish(schema.events()).await?;
        }

        Ok(())
    }
}
