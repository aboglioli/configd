use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::{Error, Schema, SchemaId, SchemaRepository};

pub struct InMemSchemaRepository {
    items: RwLock<HashMap<String, Schema>>,
}

impl InMemSchemaRepository {
    pub fn new() -> InMemSchemaRepository {
        InMemSchemaRepository {
            items: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl SchemaRepository for InMemSchemaRepository {
    async fn find_by_id(&self, id: &SchemaId) -> Result<Option<Schema>, Error> {
        Ok(self.items.read().await.get(id.value()).cloned())
    }

    async fn save(&self, schema: &mut Schema) -> Result<(), Error> {
        self.items
            .write()
            .await
            .insert(schema.id().to_string(), schema.clone());

        Ok(())
    }

    async fn delete(&self, id: &SchemaId) -> Result<(), Error> {
        self.items.write().await.remove(id.value());

        Ok(())
    }
}
