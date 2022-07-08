use core_lib::events::LocalEventBus;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    config::{Config, Storage},
    domain::{Error, SchemaRepository},
    infrastructure::{InMemSchemaRepository, SQLiteSchemaRepository, Sha256Hasher},
};

pub struct Container {
    pub event_publisher: Arc<LocalEventBus>,
    pub hasher: Arc<Sha256Hasher>,
    pub schema_repository: Arc<dyn SchemaRepository + Sync + Send>,
}

impl Container {
    pub async fn build(config: &Config) -> Result<Container, Error> {
        let event_publisher = Arc::new(LocalEventBus::new());

        let hasher = Arc::new(Sha256Hasher::new());

        let schema_repository: Arc<dyn SchemaRepository + Sync + Send> = match config.storage {
            Storage::InMem => Arc::new(InMemSchemaRepository::new()),
            Storage::SQLite => {
                let sqlite_pool = SqlitePool::connect("configd.db")
                    .await
                    .map_err(Error::Database)?;
                Arc::new(SQLiteSchemaRepository::new(sqlite_pool).await?)
            }
        };

        Ok(Container {
            event_publisher,
            hasher,
            schema_repository,
        })
    }
}
