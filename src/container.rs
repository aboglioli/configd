use std::sync::Arc;

use crate::{
    config::{Config, Storage},
    domain::{Error, JsonPropBuilder},
    infrastructure::InMemSchemaRepository,
};

pub struct Container {
    pub prop_builder: Arc<JsonPropBuilder>,
    pub schema_repository: Arc<InMemSchemaRepository>,
}

impl Container {
    pub fn build(config: &Config) -> Result<Container, Error> {
        let prop_builder = JsonPropBuilder::new();
        let schema_repository = match config.storage {
            Storage::InMem => InMemSchemaRepository::new(),
        };

        Ok(Container {
            prop_builder: Arc::new(prop_builder),
            schema_repository: Arc::new(schema_repository),
        })
    }
}
