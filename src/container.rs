use core_lib::events::LocalEventBus;
use std::sync::Arc;

use crate::{
    config::{Config, Storage},
    domain::Error,
    infrastructure::InMemSchemaRepository,
};

pub struct Container {
    pub event_publisher: Arc<LocalEventBus>,
    pub schema_repository: Arc<InMemSchemaRepository>,
}

impl Container {
    pub fn build(config: &Config) -> Result<Container, Error> {
        let event_publisher = Arc::new(LocalEventBus::new());
        let schema_repository = Arc::new(match config.storage {
            Storage::InMem => InMemSchemaRepository::new(),
        });

        Ok(Container {
            event_publisher,
            schema_repository,
        })
    }
}
