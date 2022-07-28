use sqlx::{PgPool, SqlitePool};
use std::sync::Arc;

use crate::{
    application::{CleanConfigAccesses, RevalidateConfigs},
    config::{Config, Storage},
    domain::{errors::Error, events::Subscriber, schemas::SchemaRepository},
    infrastructure::{
        InMemSchemaRepository, LocalEventBus, PostgresSchemaRepository, SQLiteSchemaRepository,
    },
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
            Storage::Postgres { ref url } => {
                let postgres_pool = PgPool::connect(url).await.map_err(Error::Database)?;
                Arc::new(PostgresSchemaRepository::new(postgres_pool).await?)
            }
        };

        // Handlers
        let clean_config_accesses =
            CleanConfigAccesses::new(event_publisher.clone(), schema_repository.clone());

        let revalidate_configs =
            RevalidateConfigs::new(event_publisher.clone(), schema_repository.clone());

        // Subscriptions
        event_publisher
            .subscribe("config.accessed", Box::new(clean_config_accesses))
            .await
            .unwrap();
        event_publisher
            .subscribe("schema.root_prop_changed", Box::new(revalidate_configs))
            .await
            .unwrap();

        Ok(Container {
            event_publisher,
            schema_repository,
        })
    }
}
