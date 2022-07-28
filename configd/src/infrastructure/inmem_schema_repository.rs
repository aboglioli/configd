use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::{
    errors::Error,
    schemas::{Schema, SchemaRepository},
    shared::{Id, Page},
};

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
    async fn find(&self, offset: Option<u64>, limit: Option<u64>) -> Result<Page<Schema>, Error> {
        let offset = offset.unwrap_or(0);
        let mut limit = limit.unwrap_or(10);
        if limit > 100 {
            limit = 100;
        }

        let items = self.items.read().await;

        Page::new(
            offset,
            limit,
            items.len() as u64,
            items
                .values()
                .skip(offset as usize)
                .take(limit as usize)
                .map(Clone::clone)
                .collect(),
        )
    }

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
}
