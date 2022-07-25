use core_lib::events::{LocalEventBus, Subscriber};
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    application::CleanConfigAccesses,
    config::{Config, Storage},
    domain::{Error, SchemaRepository},
    infrastructure::{InMemSchemaRepository, SQLiteSchemaRepository},
};

pub struct Container {
    pub event_publisher: Arc<LocalEventBus>,
    pub schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl Container {
    pub async fn build(config: &Config) -> Result<Container, Error> {
        let event_publisher = Arc::new(LocalEventBus::new());

        let schema_repository: Arc<dyn SchemaRepository + Sync + Send> = match config.storage {
            Storage::InMem => Arc::new(InMemSchemaRepository::new()),
            Storage::SQLite { ref filename } => {
                let sqlite_pool = SqlitePool::connect(filename)
                    .await
                    .map_err(Error::Database)?;
                Arc::new(SQLiteSchemaRepository::new(sqlite_pool).await?)
            }
        };

        let clean_config_accesses = CleanConfigAccesses::new(schema_repository.clone());
        event_publisher
            .subscribe("config.accessed".into(), Box::new(clean_config_accesses))
            .await
            .unwrap();

        Ok(Container {
            event_publisher,
            schema_repository,
        })
    }
}
