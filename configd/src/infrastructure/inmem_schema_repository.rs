use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::{Error, Id, Schema, SchemaRepository};

pub struct InMemSchemaRepository {
    items: RwLock<HashMap<Id, Schema>>,
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
    async fn find_by_id(&self, id: &Id) -> Result<Option<Schema>, Error> {
        Ok(self.items.read().await.get(id).cloned())
    }

    async fn exists(&self, id: &Id) -> Result<bool, Error> {
        Ok(self.items.read().await.contains_key(id))
    }

    async fn save(&self, schema: &mut Schema) -> Result<(), Error> {
        self.items
            .write()
            .await
            .insert(schema.id().clone(), schema.clone());

        Ok(())
    }

    async fn delete(&self, id: &Id) -> Result<(), Error> {
        self.items.write().await.remove(id);

        Ok(())
    }
}
