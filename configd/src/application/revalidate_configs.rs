use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::{
    errors::Error,
    events::{Event, Handler},
    schemas::{SchemaRepository, SchemaRootPropChanged},
    shared::Id,
};

pub struct RevalidateConfigs {
    schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl RevalidateConfigs {
    pub fn new(schema_repository: Arc<dyn SchemaRepository + Sync + Send>) -> RevalidateConfigs {
        RevalidateConfigs { schema_repository }
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
                .await
                .unwrap()
                .unwrap();

            schema.revalidate_configs()?;

            self.schema_repository.save(&mut schema).await.unwrap();
        }

        Ok(())
    }
}
