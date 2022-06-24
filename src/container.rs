use std::sync::Arc;

use crate::{
    config::{Config, Storage},
    domain::Error,
    infrastructure::{InMemConfigRepository, InMemSchemaRepository, JsonPropConverter},
};

pub struct Container {
    pub config_repository: Arc<InMemConfigRepository>,
    pub prop_converter: Arc<JsonPropConverter>,
    pub schema_repository: Arc<InMemSchemaRepository>,
}

impl Container {
    pub fn build(config: &Config) -> Result<Container, Error> {
        let config_repository = InMemConfigRepository::new();
        let prop_converter = JsonPropConverter::new();
        let schema_repository = match config.storage {
            Storage::InMem => InMemSchemaRepository::new(),
        };

        Ok(Container {
            config_repository: Arc::new(config_repository),
            prop_converter: Arc::new(prop_converter),
            schema_repository: Arc::new(schema_repository),
        })
    }
}
