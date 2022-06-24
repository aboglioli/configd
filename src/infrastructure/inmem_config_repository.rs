use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::{Config, ConfigRepository, Error, Id};

pub struct InMemConfigRepository {
    items: RwLock<HashMap<String, Config>>,
}

impl InMemConfigRepository {
    pub fn new() -> InMemConfigRepository {
        InMemConfigRepository {
            items: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl ConfigRepository for InMemConfigRepository {
    async fn find_by_id(&self, schema_id: &Id, config_id: &Id) -> Result<Option<Config>, Error> {
        Ok(self
            .items
            .read()
            .await
            .get(&format!("{}:{}", schema_id.value(), config_id.value()))
            .cloned())
    }

    async fn exists(&self, schema_id: &Id, id: &Id) -> Result<bool, Error> {
        Ok(self
            .items
            .read()
            .await
            .contains_key(&format!("{}:{}", schema_id.value(), id.value())))
    }

    async fn save(&self, config: &mut Config) -> Result<(), Error> {
        self.items.write().await.insert(
            format!("{}:{}", config.schema_id().value(), config.id().value()),
            config.clone(),
        );

        Ok(())
    }

    async fn delete(&self, schema_id: &Id, id: &Id) -> Result<(), Error> {
        self.items
            .write()
            .await
            .remove(&format!("{}:{}", schema_id.value(), id.value()));

        Ok(())
    }
}
