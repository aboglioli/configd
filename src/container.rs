use std::sync::Arc;

use crate::{
    config::{Config, Storage},
    domain::{Error, JsonPropConverter},
    infrastructure::InMemSchemaRepository,
};

pub struct Container {
    pub prop_converter: Arc<JsonPropConverter>,
    pub schema_repository: Arc<InMemSchemaRepository>,
}

impl Container {
    pub fn build(config: &Config) -> Result<Container, Error> {
        let prop_converter = JsonPropConverter::new();
        let schema_repository = match config.storage {
            Storage::InMem => InMemSchemaRepository::new(),
        };

        Ok(Container {
            prop_converter: Arc::new(prop_converter),
            schema_repository: Arc::new(schema_repository),
        })
    }
}
